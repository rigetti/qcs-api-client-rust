use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify, RwLock};

use super::{settings::AuthServer, ClientConfiguration, TokenError};
#[cfg(feature = "tracing-config")]
use crate::tracing_configuration::TracingConfiguration;
#[cfg(feature = "tracing")]
use urlpattern::UrlPatternMatchInput;

/// A single type containing an access token and an associated refresh token.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct Tokens {
    /// The `Bearer` token to include in the `Authorization` header.
    pub bearer_access_token: String,
    /// The token used to refresh the access token.
    pub refresh_token: String,
    /// The server that issued the tokens.
    pub auth_server: AuthServer,
}

/// A wrapper for [`Tokens`] that provides thread-safe access to the inner tokens.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct TokenDispatcher {
    lock: Arc<RwLock<Tokens>>,
    refreshing: Arc<Mutex<bool>>,
    notify_refreshed: Arc<Notify>,
}

impl From<Tokens> for TokenDispatcher {
    fn from(value: Tokens) -> Self {
        Self {
            lock: Arc::new(RwLock::new(value)),
            refreshing: Arc::new(Mutex::new(false)),
            notify_refreshed: Arc::new(Notify::new()),
        }
    }
}

impl TokenDispatcher {
    /// Executes a user-provided closure on a reference to the `Tokens` instance managed by the
    /// dispatcher.
    ///
    /// This function locks the mutex, safely exposing the protected `Tokens` instance to the provided closure `f`.
    /// It is designed to allow safe and controlled access to the `Tokens` instance for reading its state.
    ///
    /// # Parameters
    /// - `f`: A closure that takes a reference to `Tokens` and returns a value of type `O`. The closure is called
    ///   with the `Tokens` instance as an argument once the mutex is successfully locked.
    pub async fn use_tokens<F, O>(&self, f: F) -> O
    where
        F: FnOnce(&Tokens) -> O + Send,
    {
        let tokens = self.lock.read().await;
        f(&tokens)
    }

    /// Get a copy of the current access token.
    #[must_use]
    pub async fn tokens(&self) -> Tokens {
        self.use_tokens(Clone::clone).await
    }

    /// Refreshes the tokens. Readers will be blocked until the refresh is complete.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn refresh(&self) -> Result<Tokens, TokenError> {
        self.managed_refresh(Self::perform_refresh).await
    }

    /// If tokens are already being refreshed, wait and return the updated tokens. Otherwise, run
    /// ``refresh_fn``.
    async fn managed_refresh<F, Fut>(&self, refresh_fn: F) -> Result<Tokens, TokenError>
    where
        F: FnOnce(Arc<RwLock<Tokens>>) -> Fut + Send,
        Fut: std::future::Future<Output = Result<Tokens, TokenError>> + Send,
    {
        let mut is_refreshing = self.refreshing.lock().await;

        if *is_refreshing {
            drop(is_refreshing);
            self.notify_refreshed.notified().await;
            return Ok(self.tokens().await);
        }

        *is_refreshing = true;
        drop(is_refreshing);

        let result = refresh_fn(self.lock.clone()).await;

        *self.refreshing.lock().await = false;
        self.notify_refreshed.notify_waiters();

        result
    }

    /// Refreshes the tokens. Readers will be blocked until the refresh is complete.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    async fn perform_refresh(lock: Arc<RwLock<Tokens>>) -> Result<Tokens, TokenError> {
        let mut tokens = lock.write().await;
        let auth_server = &tokens.auth_server;

        let token_url = format!("{}/v1/token", auth_server.issuer());
        let data = TokenRefreshRequest::new(auth_server.client_id(), &tokens.refresh_token);
        let resp = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?
            .post(token_url)
            .form(&data)
            .send()
            .await?;

        let response_data: TokenResponse = resp.error_for_status()?.json().await?;
        tokens.bearer_access_token = response_data.access_token;
        tokens.refresh_token = response_data.refresh_token;
        Ok(tokens.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TokenRefreshRequest<'a> {
    grant_type: &'static str,
    client_id: &'a str,
    refresh_token: &'a str,
}

impl<'a> TokenRefreshRequest<'a> {
    pub(super) const fn new(client_id: &'a str, refresh_token: &'a str) -> TokenRefreshRequest<'a> {
        Self {
            grant_type: "refresh_token",
            client_id,
            refresh_token,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub(super) struct TokenResponse {
    pub(super) refresh_token: String,
    pub(super) access_token: String,
}

/// Get and refresh access tokens
#[async_trait::async_trait]
pub trait TokenRefresher: Clone + Send {
    /// The type to be returned in the event of a error during getting or
    /// refreshing an access token
    type Error;

    /// Get the current access token
    async fn get_access_token(&self) -> Result<String, Self::Error>;

    /// Get a fresh access token
    async fn refresh_access_token(&self) -> Result<String, Self::Error>;

    /// Get the base URL for requests
    #[cfg(feature = "tracing")]
    fn base_url(&self) -> &str;

    /// Get the tracing configuration
    #[cfg(feature = "tracing-config")]
    fn tracing_configuration(&self) -> Option<&TracingConfiguration>;

    /// Returns whether the given URL should be traced. Following
    /// [`TracingConfiguration::is_enabled`], this defaults to `true`.
    #[cfg(feature = "tracing")]
    #[allow(clippy::needless_return)]
    fn should_trace(&self, url: &UrlPatternMatchInput) -> bool {
        #[cfg(not(feature = "tracing-config"))]
        {
            let _ = url;
            return true;
        }

        #[cfg(feature = "tracing-config")]
        self.tracing_configuration()
            .map_or(true, |config| config.is_enabled(url))
    }
}

#[async_trait::async_trait]
impl TokenRefresher for ClientConfiguration {
    type Error = TokenError;

    async fn refresh_access_token(&self) -> Result<String, Self::Error> {
        Ok(self.refresh().await?.bearer_access_token)
    }

    async fn get_access_token(&self) -> Result<String, Self::Error> {
        self.get_bearer_access_token().await
    }

    #[cfg(feature = "tracing")]
    fn base_url(&self) -> &str {
        &self.grpc_api_url
    }

    #[cfg(feature = "tracing-config")]
    fn tracing_configuration(&self) -> Option<&TracingConfiguration> {
        self.tracing_configuration.as_ref()
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    use httpmock::prelude::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_tokens_blocked_during_refresh() {
        let mock_server = MockServer::start_async().await;
        let auth_server = AuthServer::new("client_id".to_string(), mock_server.base_url());

        let original_tokens = Tokens {
            bearer_access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            auth_server: auth_server.clone(),
        };

        let dispatcher: TokenDispatcher = original_tokens.clone().into();
        let dispatcher_clone1 = dispatcher.clone();
        let dispatcher_clone2 = dispatcher.clone();

        let refresh_duration = Duration::from_secs(3);

        let issuer_mock = mock_server
            .mock_async(|when, then| {
                when.method(POST).path("/v1/token");

                then.status(200)
                    .delay(Duration::from_secs(3))
                    .json_body_obj(&TokenResponse {
                        access_token: "new_access".to_string(),
                        refresh_token: "new_refresh".to_string(),
                    });
            })
            .await;

        let start_write = Instant::now();
        let write_future = tokio::spawn(async move { dispatcher_clone1.refresh().await.unwrap() });

        let start_read = Instant::now();
        let read_future = tokio::spawn(async move { dispatcher_clone2.tokens().await });

        let _ = write_future.await.unwrap();
        let read_result = read_future.await.unwrap();

        let write_duration = start_write.elapsed();
        let read_duration = start_read.elapsed();

        issuer_mock.assert_async().await;

        assert!(
            write_duration >= refresh_duration,
            "Write operation did not take enough time"
        );
        assert!(
            read_duration >= refresh_duration,
            "Read operation was not blocked by the write operation"
        );
        assert_eq!(read_result.bearer_access_token, "new_access");
        assert_eq!(read_result.refresh_token, "new_refresh");
    }
}
