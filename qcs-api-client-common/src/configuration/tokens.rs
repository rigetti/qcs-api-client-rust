use std::{pin::Pin, sync::Arc};

use futures::Future;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio::sync::{Mutex, Notify, RwLock};

use super::{
    secrets::Secrets, settings::AuthServer, ClientConfiguration, ConfigSource, TokenError,
    QCS_AUDIENCE,
};
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
        if self.refresh_token.is_empty() {
            return Err(TokenError::NoRefreshToken);
        }
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
/// A pair of Client ID and Client Secret, used to request an OAuth Client Credentials Grant
pub struct ClientCredentials {
    /// The client ID
    pub client_id: String,
    /// The client secret.
    pub client_secret: String,
}

impl ClientCredentials {
    #[must_use]
    /// Construct a new [`ClientCredentials`]
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
        &self,
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "python", derive(pyo3::FromPyObject))]
/// Specifies the [OAuth2 grant type](https://oauth.net/2/grant-types/) to use, along with the data
/// needed to request said grant type.
pub enum OAuthGrant {
    /// Credentials that can be used to use with the [Refresh Token grant type](https://oauth.net/2/grant-types/refresh-token/).
    RefreshToken(RefreshToken),
    /// Payload that can be used to use the [Client Credentials grant type](https://oauth.net/2/grant-types/client-credentials/).
    ClientCredentials(ClientCredentials),
    /// Defers to a user provided function for access token requests.
    ExternallyManaged(ExternallyManaged),
}

impl From<ExternallyManaged> for OAuthGrant {
    fn from(v: ExternallyManaged) -> Self {
        Self::ExternallyManaged(v)
    }
}

impl From<ClientCredentials> for OAuthGrant {
    fn from(v: ClientCredentials) -> Self {
        Self::ClientCredentials(v)
    }
}

impl From<RefreshToken> for OAuthGrant {
    fn from(v: RefreshToken) -> Self {
        Self::RefreshToken(v)
    }
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
            Self::ExternallyManaged(tokens) => tokens
                .request_access_token(auth_server)
                .await
                .map_err(|e| TokenError::ExternallyManaged(e.to_string())),
        }
    }
}

/// Manages the `OAuth2` authorization process and token lifecycle for accessing the QCS API.
///
/// This struct encapsulates the necessary information to request an access token
/// from an authorization server, including the `OAuth2` grant type and any associated
/// credentials or payload data.
///
/// # Fields
///
/// * `payload` - The `OAuth2` grant type and associated data that will be used to request an access token.
/// * `access_token` - The access token currently in use, if any. If no token has been provided or requested yet, this will be `None`.
/// * `auth_server` - The authorization server responsible for issuing tokens.
#[derive(Clone, Debug)]
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

    /// Initialize a new set of [`Credentials`] using an [`ExternallyManaged`].
    ///
    /// Optionally include an `access_token`, if not included, then one can be requested
    /// with [`Self::request_access_token`].
    #[must_use]
    pub const fn from_externally_managed(
        tokens: ExternallyManaged,
        auth_server: AuthServer,
        access_token: Option<String>,
    ) -> Self {
        Self::new(
            OAuthGrant::ExternallyManaged(tokens),
            auth_server,
            access_token,
        )
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
    pub async fn refresh(
        &self,
        source: &ConfigSource,
        profile: &str,
    ) -> Result<OAuthSession, TokenError> {
        self.managed_refresh(Self::perform_refresh, source, profile)
            .await
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
    async fn managed_refresh<F, Fut>(
        &self,
        refresh_fn: F,
        source: &ConfigSource,
        profile: &str,
    ) -> Result<OAuthSession, TokenError>
    where
        F: FnOnce(Arc<RwLock<OAuthSession>>) -> Fut + Send,
        Fut: Future<Output = Result<OAuthSession, TokenError>> + Send,
    {
        let mut is_refreshing = self.refreshing.lock().await;

        if *is_refreshing {
            drop(is_refreshing);
            self.notify_refreshed.notified().await;
            return Ok(self.tokens().await);
        }

        *is_refreshing = true;
        drop(is_refreshing);

        let oauth_session = refresh_fn(self.lock.clone()).await?;

        // If the config source is a file, write the new access token to the file
        if let ConfigSource::File {
            settings_path: _,
            secrets_path,
        } = source
        {
            if !Secrets::is_read_only(secrets_path).await? {
                let now = OffsetDateTime::now_utc();
                Secrets::write_access_token(
                    secrets_path,
                    profile,
                    oauth_session.access_token()?,
                    now,
                )
                .await?;
            }
        }

        *self.refreshing.lock().await = false;
        self.notify_refreshed.notify_waiters();
        Ok(oauth_session)
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

pub(crate) type RefreshResult =
    Pin<Box<dyn Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>> + Send>>;

/// A function that asynchronously refreshes a token.
pub type RefreshFunction = Box<dyn (Fn(AuthServer) -> RefreshResult) + Send + Sync>;

/// A struct that manages access tokens by utilizing a user-provided refresh function.
///
/// The [`ExternallyManaged`] struct allows users to define custom logic for
/// fetching or refreshing access tokens.
#[derive(Clone)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct ExternallyManaged {
    refresh_function: Arc<RefreshFunction>,
}

impl ExternallyManaged {
    /// Creates a new [`ExternallyManaged`] instance from a [`RefreshFunction`].
    ///
    /// Consider using [`ExternallyManaged::from_async`], and [`ExternallyManaged::from_sync`], if
    /// they better fit your use case.
    ///
    /// # Arguments
    ///
    /// * `refresh_function` - A function or closure that asynchronously refreshes a token.
    ///
    /// # Example
    ///
    /// ```
    /// use qcs_api_client_common::configuration::{AuthServer, ExternallyManaged, TokenError};
    /// use std::future::Future;
    /// use std::pin::Pin;
    /// use std::boxed::Box;
    /// use std::error::Error;
    ///
    /// async fn example_refresh_function(_auth_server: AuthServer) -> Result<String, Box<dyn Error
    /// + Send + Sync>> {
    ///     Ok("new_token_value".to_string())
    /// }
    /// let token_manager = ExternallyManaged::new(|auth_server| Box::pin(example_refresh_function(auth_server)));
    /// ```
    pub fn new(
        refresh_function: impl Fn(AuthServer) -> RefreshResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            refresh_function: Arc::new(Box::new(refresh_function)),
        }
    }

    /// Constructs a new [`ExternallyManaged`] instance using an async function or closure.
    ///
    /// This method simplifies the creation of the [`ExternallyManaged`] instance by handling
    /// the boxing and pinning of the future internally.
    ///
    /// # Arguments
    ///
    /// * `refresh_function` - An async function or closure that returns a [`Future`] which, when awaited,
    ///   produces a [`Result<String, TokenError>`].
    ///
    /// # Example
    ///
    /// ```
    /// use qcs_api_client_common::configuration::{AuthServer, ExternallyManaged, TokenError};
    /// use tokio::runtime::Runtime;
    /// use std::error::Error;
    ///
    /// async fn example_refresh_function(_auth_server: AuthServer) -> Result<String, Box<dyn Error
    /// + Send + Sync>> {
    ///     Ok("new_token_value".to_string())
    /// }
    ///
    /// let token_manager = ExternallyManaged::from_async(example_refresh_function);
    ///
    /// let rt = Runtime::new().unwrap();
    /// rt.block_on(async {
    ///     match token_manager.request_access_token(&AuthServer::default()).await {
    ///         Ok(token) => println!("Token: {}", token),
    ///         Err(e) => println!("Failed to refresh token: {:?}", e),
    ///     }
    /// });
    /// ```
    pub fn from_async<F, Fut>(refresh_function: F) -> Self
    where
        F: Fn(AuthServer) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'static,
    {
        Self {
            refresh_function: Arc::new(Box::new(move |auth_server| {
                Box::pin(refresh_function(auth_server))
            })),
        }
    }

    /// Constructs a new [`ExternallyManaged`] instance using a synchronous function.
    ///
    /// The synchronous function is wrapped in an async block to fit the expected signature.
    ///
    /// # Arguments
    ///
    /// * `refresh_function` - A synchronous function that returns a [`Result<String, TokenError>`].
    ///
    /// # Example
    ///
    /// ```
    /// use qcs_api_client_common::configuration::{AuthServer, ExternallyManaged, TokenError};
    /// use tokio::runtime::Runtime;
    /// use std::error::Error;
    ///
    /// fn example_sync_refresh_function(_auth_server: AuthServer) -> Result<String, Box<dyn Error
    /// + Send + Sync>> {
    ///     Ok("sync_token_value".to_string())
    /// }
    ///
    /// let token_manager = ExternallyManaged::from_sync(example_sync_refresh_function);
    ///
    /// let rt = Runtime::new().unwrap();
    /// rt.block_on(async {
    ///     match token_manager.request_access_token(&AuthServer::default()).await {
    ///         Ok(token) => println!("Token: {}", token),
    ///         Err(e) => println!("Failed to refresh token: {:?}", e),
    ///     }
    /// });
    /// ```
    pub fn from_sync(
        refresh_function: impl Fn(AuthServer) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            refresh_function: Arc::new(Box::new(move |auth_server| {
                let result = refresh_function(auth_server);
                Box::pin(async move { result })
            })),
        }
    }

    /// Request an updated access token using the provided refresh function.
    ///
    /// # Errors
    ///
    /// Errors are propagated from the refresh function.
    pub async fn request_access_token(
        &self,
        auth_server: &AuthServer,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        (self.refresh_function)(auth_server.clone()).await
    }
}

impl std::fmt::Debug for ExternallyManaged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ExternallyManaged")
            .field(
                "refresh_function",
                &"Fn() -> Pin<Box<dyn Future<Output = Result<String, TokenError>> + Send>>",
            )
            .finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct TokenRefreshRequest<'a> {
    grant_type: &'static str,
    client_id: &'a str,
    refresh_token: &'a str,
}

impl<'a> TokenRefreshRequest<'a> {
    pub(super) const fn new(client_id: &'a str, refresh_token: &'a str) -> Self {
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
    pub(super) const fn new(client_id: &'a str, client_secret: &'a str) -> Self {
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

    /// Get and validate the current access token, refreshing it if it doesn't exist or is invalid.
    async fn validated_access_token(&self) -> Result<String, Self::Error>;

    /// Get the current access token, if any
    async fn get_access_token(&self) -> Result<Option<String>, Self::Error>;

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

    async fn validated_access_token(&self) -> Result<String, Self::Error> {
        self.get_bearer_access_token().await
    }

    async fn refresh_access_token(&self) -> Result<String, Self::Error> {
        Ok(self.refresh().await?.access_token()?.to_string())
    }

    async fn get_access_token(&self) -> Result<Option<String>, Self::Error> {
        Ok(Some(
            self.oauth_session().await?.access_token()?.to_string(),
        ))
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
    use rstest::rstest;
    use time::format_description::well_known::Rfc3339;
    use tokio::time::Instant;
    use toml_edit::DocumentMut;

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
        let write_future = tokio::spawn(async move {
            dispatcher_clone1
                .refresh(&ConfigSource::Default, "")
                .await
                .unwrap()
        });

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

    #[rstest]
    fn test_qcs_secrets_readonly(
        #[values(
            (Some("TRUE"), true),
            (Some("tRue"), true),
            (Some("true"), true),
            (Some("YES"), true),
            (Some("yEs"), true),
            (Some("yes"), true),
            (Some("1"), true),
            (Some("2"), false),
            (Some("other"), false),
            (Some(""), false),
            (None, false),
        )]
        read_only_values: (Option<&str>, bool),
        #[values(true, false)] read_only_perm: bool,
    ) {
        let (maybe_read_only_env, env_is_read_only) = read_only_values;
        let expected_update = !env_is_read_only && !read_only_perm;
        figment::Jail::expect_with(|jail| {
            let profile_name = "test";
            let initial_access_token = "initial_access_token";
            let initial_refresh_token = "initial_refresh_token";

            let initial_secrets_file_contents = format!(
                r#"
[credentials]
[credentials.{profile_name}]
[credentials.{profile_name}.token_payload]
access_token = "{initial_access_token}"
expires_in = 3600
id_token = "id_token"
refresh_token = "{initial_refresh_token}"
scope = "offline_access openid profile email"
token_type = "Bearer"
updated_at = "2024-01-01T00:00:00Z"
"#
            );

            // Create a temporary secrets file
            let secrets_path = "secrets.toml";
            jail.create_file(secrets_path, initial_secrets_file_contents.as_str())
                .expect("should create test secrets.toml");

            if read_only_perm {
                let mut permissions = std::fs::metadata(secrets_path)
                    .expect("Should be able to get file metadata")
                    .permissions();
                permissions.set_readonly(true);
                std::fs::set_permissions(secrets_path, permissions)
                    .expect("Should be able to set file permissions");
            }

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mock_server = MockServer::start_async().await;

                // Set up the mock token endpoint
                let new_access_token = "new_access_token";
                let issuer_mock = mock_server
                    .mock_async(|when, then| {
                        when.method(POST).path("/v1/token");
                        then.status(200).json_body_obj(&TokenResponse {
                            access_token: new_access_token.to_string(),
                            refresh_token: initial_refresh_token.to_string(),
                        });
                    })
                    .await;

                // Create tokens and dispatcher
                let original_tokens = OAuthSession::from_refresh_token(
                    RefreshToken::new(initial_refresh_token.to_string()),
                    AuthServer::new("client_id".to_string(), mock_server.base_url()),
                    Some(initial_refresh_token.to_string()),
                );
                let dispatcher: TokenDispatcher = original_tokens.into();

                // Test with QCS_SECRETS_READ_ONLY set first
                jail.set_env("QCS_SECRETS_FILE_PATH", "secrets.toml");
                jail.set_env("QCS_PROFILE_NAME", "test");
                if let Some(read_only_env) = maybe_read_only_env {
                    jail.set_env("QCS_SECRETS_READ_ONLY", read_only_env);
                }

                let before_refresh = OffsetDateTime::now_utc();

                dispatcher
                    .refresh(
                        &ConfigSource::File {
                            settings_path: "".into(),
                            secrets_path: "secrets.toml".into(),
                        },
                        profile_name,
                    )
                    .await
                    .unwrap();

                issuer_mock.assert_async().await;

                // Verify the file was not updated if QCS_SECRETS_READ_ONLY is set truthy
                let content = std::fs::read_to_string("secrets.toml").unwrap();
                if !expected_update {
                    assert!(
                        content.eq(initial_secrets_file_contents.as_str()),
                        "File should not be updated when QCS_SECRETS_READ_ONLY is set or file permissions are read-only"
                    );
                    return;
                }

                // Verify the file was updated
                let mut toml = std::fs::read_to_string(secrets_path)
                    .unwrap()
                    .parse::<DocumentMut>()
                    .unwrap();

                let token_payload = toml
                    .get_mut("credentials")
                    .and_then(|credentials| {
                        credentials.get_mut(profile_name)?.get_mut("token_payload")
                    })
                    .expect("Should be able to get token_payload table");

                assert_eq!(
                    token_payload.get("access_token").unwrap().as_str().unwrap(),
                    new_access_token
                );

                assert!(
                    OffsetDateTime::parse(
                        token_payload.get("updated_at").unwrap().as_str().unwrap(),
                        &Rfc3339
                    )
                    .unwrap()
                        > before_refresh
                );

                let content = std::fs::read_to_string("secrets.toml").unwrap();
                assert!(
                content.contains("new_access_token"),
                "File should be updated with new access token when QCS_SECRETS_READ_ONLY is not set or is set but disabled, and file permissions allow writing"
                );
            });
            Ok(())
        });
    }
}
