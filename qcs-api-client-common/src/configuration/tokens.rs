use std::sync::Arc;

use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify, RwLock};

use super::{settings::AuthServer, ClientConfiguration, TokenError, QCS_AUDIENCE};
#[cfg(feature = "tracing-config")]
use crate::tracing_configuration::TracingConfiguration;
#[cfg(feature = "tracing")]
use urlpattern::UrlPatternMatchInput;

/// A single type containing an access token and an associated refresh token.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct RefreshToken {
    /// The token used to refresh the access token.
    pub refresh_token: String,
}

impl RefreshToken {
    /// Create a new [`RefreshToken`] with the given refresh token.
    #[must_use]
    pub const fn new(refresh_token: String) -> Self {
        Self { refresh_token }
    }

    /// Request and return a new access token from the given authorization server using this refresh token.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn request_access_token(
        &mut self,
        auth_server: &AuthServer,
    ) -> Result<String, TokenError> {
        let token_url = format!("{}/v1/token", auth_server.issuer());
        let data = TokenRefreshRequest::new(auth_server.client_id(), &self.refresh_token);
        let resp = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?
            .post(token_url)
            .form(&data)
            .send()
            .await?;

        let response_data: TokenResponse = resp.error_for_status()?.json().await?;
        self.refresh_token = response_data.refresh_token;
        Ok(response_data.access_token)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl ClientCredentials {
    pub const fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    /// Get the client ID.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the client secret.
    #[must_use]
    pub fn client_secret(&self) -> &str {
        &self.client_secret
    }

    /// Request and return an access token from the given auth server using this set of client credentials.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn request_access_token(
        &mut self,
        auth_server: &AuthServer,
    ) -> Result<String, TokenError> {
        let request = ClientCredentialsRequest::new(&self.client_id, &self.client_secret);
        let url = format!("{}/v1/token", auth_server.issuer());

        // let credentials = format!("{}:{}", self.auth_server.client_id(), self.client_secret);
        // let encoded_credentials = base64::encode(credentials);
        // let authorization_value = format!("Basic {}", encoded_credentials);
        let mut headers = HeaderMap::new();
        // headers.insert(AUTHORIZATION, HeaderValue::from_str(&authorization_value)?);
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let response = client
            .post(url)
            .headers(headers)
            .form(&request)
            .send()
            .await?;

        response.error_for_status_ref()?;

        let response_body: TokenResponse = response.json().await?;

        Ok(response_body.access_token)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "python", derive(pyo3::FromPyObject))]
/// Specifies the [OAuth2 grant type](https://oauth.net/2/grant-types/) to use, along with the data
/// needed to request said grant type.
pub enum OAuthGrant {
    /// Credentials that can be used to use with the [Refresh Token grant type](https://oauth.net/2/grant-types/refresh-token/).
    RefreshToken(RefreshToken),
    /// Payload that can be used to use the [Client Credentials grant type](https://oauth.net/2/grant-types/client-credentials/).
    ClientCredentials(ClientCredentials),
}

impl OAuthGrant {
    /// Request a new access token from the given issuer using this grant type and payload.
    async fn request_access_token(
        &mut self,
        auth_server: &AuthServer,
    ) -> Result<String, TokenError> {
        match self {
            Self::RefreshToken(tokens) => tokens.request_access_token(auth_server).await,
            Self::ClientCredentials(tokens) => tokens.request_access_token(auth_server).await,
        }
    }
}

/// Manages the OAuth2 authorization process and token lifecycle for accessing the QCS API.
///
/// This struct encapsulates the necessary information to request an access token
/// from an authorization server, including the OAuth2 grant type and any associated
/// credentials or payload data.
///
/// # Fields
///
/// * `payload` - The OAuth2 grant type and associated data that will be used to request an access token.
/// * `access_token` - The access token currently in use, if any. If no token has been provided or requested yet, this will be `None`.
/// * `auth_server` - The authorization server responsible for issuing tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct OAuthSession {
    /// The grant type to use to request an access token.
    payload: OAuthGrant,
    /// The access token that is currently in use. None if no token has been requested yet.
    access_token: Option<String>,
    /// The [`AuthServer`] that issues the tokens.
    auth_server: AuthServer,
}

impl OAuthSession {
    /// Initialize a new set of [`Credentials`] using a [`GrantPayload`].
    ///
    /// Optionally include an `access_token`, if not included, then one can be requested
    /// with [`Self::request_access_token`].
    #[must_use]
    pub const fn new(
        payload: OAuthGrant,
        auth_server: AuthServer,
        access_token: Option<String>,
    ) -> Self {
        Self {
            payload,
            access_token,
            auth_server,
        }
    }

    /// Initialize a new set of [`Credentials`] using a [`RefreshToken`].
    ///
    /// Optionally include an `access_token`, if not included, then one can be requested
    /// with [`Self::request_access_token`].
    #[must_use]
    pub const fn from_refresh_token(
        tokens: RefreshToken,
        auth_server: AuthServer,
        access_token: Option<String>,
    ) -> Self {
        Self::new(OAuthGrant::RefreshToken(tokens), auth_server, access_token)
    }

    /// Initialize a new set of [`Credentials`] using [`ClientCredentials`].
    ///
    /// Optionally include an `access_token`, if not included, then one can be requested
    /// with [`Self::request_access_token`].
    #[must_use]
    pub const fn from_client_credentials(
        tokens: ClientCredentials,
        auth_server: AuthServer,
        access_token: Option<String>,
    ) -> Self {
        Self::new(
            OAuthGrant::ClientCredentials(tokens),
            auth_server,
            access_token,
        )
    }

    /// Get the current access token.
    ///
    /// This is an unvalidated copy of the access token. Meaning it can become stale, or may
    /// even be already be stale. See [`Self::validate`] and [`Self::request_access_token`].
    ///
    /// # Errors
    ///
    /// - [`TokenError::NoAccessToken`] if there is no access token
    pub fn access_token(&self) -> Result<&str, TokenError> {
        self.access_token.as_ref().map_or_else(
            || Err(TokenError::NoAccessToken),
            |token| Ok(token.as_str()),
        )
    }

    /// Get the payload used to request an access token.
    #[must_use]
    pub const fn payload(&self) -> &OAuthGrant {
        &self.payload
    }

    /// Request and return an updated access token using these credentials.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    #[allow(clippy::missing_panics_doc)]
    pub async fn request_access_token(&mut self) -> Result<&str, TokenError> {
        let access_token = self.payload.request_access_token(&self.auth_server).await?;
        self.access_token = Some(access_token);
        Ok(self
            .access_token
            .as_ref()
            .expect("This value is set in the previous line, so it cannot be None"))
    }

    /// The [`AuthServer`] that issues the tokens.
    #[must_use]
    pub const fn auth_server(&self) -> &AuthServer {
        &self.auth_server
    }

    /// Validate the access token, returning it if it is valid, or an error describing why it is
    /// invalid.
    ///
    /// # Errors
    ///
    /// - [`TokenError::NoAccessToken`] if an access token has not been requested.
    /// - [`TokenError::InvalidAccessToken`] if the access token is invalid.
    pub fn validate(&self) -> Result<String, TokenError> {
        self.access_token().map_or_else(
            |_| Err(TokenError::NoAccessToken),
            |access_token| {
                let placeholder_key = DecodingKey::from_secret(&[]);
                let mut validation = Validation::new(Algorithm::RS256);
                validation.validate_exp = true;
                validation.leeway = 60;
                validation.set_audience(&[QCS_AUDIENCE]);
                validation.insecure_disable_signature_validation();
                jsonwebtoken::decode::<toml::Value>(access_token, &placeholder_key, &validation)
                    .map(|_| access_token.to_string())
                    .map_err(TokenError::InvalidAccessToken)
            },
        )
    }
}

/// A wrapper for [`OAuthSession`] that provides thread-safe access to the inner tokens.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct TokenDispatcher {
    lock: Arc<RwLock<OAuthSession>>,
    refreshing: Arc<Mutex<bool>>,
    notify_refreshed: Arc<Notify>,
}

impl From<OAuthSession> for TokenDispatcher {
    fn from(value: OAuthSession) -> Self {
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
        F: FnOnce(&OAuthSession) -> O + Send,
    {
        let tokens = self.lock.read().await;
        f(&tokens)
    }

    /// Get a copy of the current access token.
    #[must_use]
    pub async fn tokens(&self) -> OAuthSession {
        self.use_tokens(Clone::clone).await
    }

    /// Refreshes the tokens. Readers will be blocked until the refresh is complete.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn refresh(&self) -> Result<OAuthSession, TokenError> {
        self.managed_refresh(Self::perform_refresh).await
    }

    /// Validate the access token, returning it if it is valid, or an error describing why it is
    /// invalid.
    ///
    /// # Errors
    ///
    /// - [`TokenError::NoAccessToken`] if there is no access token
    /// - [`TokenError::InvalidAccessToken`] if the access token is invalid
    pub async fn validate(&self) -> Result<String, TokenError> {
        self.use_tokens(OAuthSession::validate).await
    }

    /// If tokens are already being refreshed, wait and return the updated tokens. Otherwise, run
    /// ``refresh_fn``.
    async fn managed_refresh<F, Fut>(&self, refresh_fn: F) -> Result<OAuthSession, TokenError>
    where
        F: FnOnce(Arc<RwLock<OAuthSession>>) -> Fut + Send,
        Fut: std::future::Future<Output = Result<OAuthSession, TokenError>> + Send,
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

    /// Refreshes the tokens. Readers will be blocked until the refresh is complete. Returns a copy
    /// of the updated [`Credentials`]
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    async fn perform_refresh(lock: Arc<RwLock<OAuthSession>>) -> Result<OAuthSession, TokenError> {
        let mut credentials = lock.write().await;
        credentials.request_access_token().await?;
        Ok(credentials.clone())
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

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ClientCredentialsRequest<'a> {
    grant_type: &'static str,
    client_id: &'a str,
    client_secret: &'a str,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub(super) const fn new(
        client_id: &'a str,
        client_secret: &'a str,
    ) -> ClientCredentialsRequest<'a> {
        Self {
            grant_type: "client_credentials",
            client_id,
            client_secret,
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
pub trait TokenRefresher: Clone + std::fmt::Debug + Send {
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
        Ok(self.refresh().await?.access_token()?.to_string())
    }

    async fn get_access_token(&self) -> Result<String, Self::Error> {
        Ok(self.oauth_session().await?.access_token()?.to_string())
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

        let original_tokens = OAuthSession::from_refresh_token(
            RefreshToken::new("refresh".to_string()),
            AuthServer::new("client_id".to_string(), mock_server.base_url()),
            None,
        );
        let dispatcher: TokenDispatcher = original_tokens.clone().into();
        let dispatcher_clone1 = dispatcher.clone();
        let dispatcher_clone2 = dispatcher.clone();

        let refresh_duration = Duration::from_secs(3);

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
        assert_eq!(read_result.access_token.as_ref().unwrap(), "new_access");
        if let OAuthGrant::RefreshToken(payload) = read_result.payload {
            assert_eq!(&payload.refresh_token, "new_refresh");
        } else {
            panic!(
                "Expected RefreshToken payload, got {:?}",
                read_result.payload
            );
        }
    }
}
