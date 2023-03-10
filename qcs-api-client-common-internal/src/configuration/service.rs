use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, ClientId, ClientSecret, Scope, TokenResponse, TokenUrl};
use qcs_api_client_common::configuration::TokenRefresher;
#[cfg(feature = "otel-tracing")]
use qcs_api_client_common::otel_tracing::TracingConfiguration;
use std::ops::Add;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;

use serde::Deserialize;

/// Credentials for an OAuth service
#[derive(Clone, Debug, Deserialize)]
pub struct OAuthClientCredentials {
    /// The client ID
    pub client_id: String,
    /// The client secret
    pub client_secret: String,
}

/// Errors encountered when loading a [`ClientConfiguration`]
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("failed to load settings: {0}")]
    Settings(#[from] config::ConfigError),
    #[error("failed to get access token manager: {0}")]
    AccessTokenManager(#[from] AccessTokenError),
}

#[derive(Debug)]
struct Token {
    current_access_token: String,
    access_token_expiration_time: SystemTime,
}

/// The configuration needed for getting access tokens for an OAuth application
/// using the [client credentials flow](https://developer.okta.com/docs/guides/implement-grant-type/clientcreds/main/#client-credentials-flow)
#[derive(Debug, Clone)]
pub struct ClientConfiguration {
    /// The OAuth client settings
    pub oauth_server_url: String,
    pub oauth_client_credentials: OAuthClientCredentials,
    token: Arc<Mutex<Token>>,
}

impl ClientConfiguration {
    /// Create a [`ClientConfiguration`] using the provided settings
    pub async fn load(
        oauth_server_url: String,
        oauth_client_credentials: OAuthClientCredentials,
    ) -> Result<Self, LoadError> {
        let (current_access_token, access_token_expiration_time) =
            ClientConfiguration::fetch_service_access_token(
                oauth_client_credentials.clone(),
                oauth_server_url.clone(),
            )
            .await?;
        Ok(Self {
            oauth_server_url,
            oauth_client_credentials,
            token: Arc::new(Mutex::new(Token {
                current_access_token,
                access_token_expiration_time,
            })),
        })
    }

    /// Return the current access token if not expired; otherwise, fetch a new one
    ///
    /// Optionally `force` the token to be refreshed even if it hasn't expired.
    async fn internal_get_access_token(&self, force: bool) -> Result<String, AccessTokenError> {
        let mut lock = self.token.lock().await;

        if force || lock.access_token_expiration_time <= SystemTime::now() {
            let (new_access_token, expiration_time) = Self::fetch_service_access_token(
                self.oauth_client_credentials.clone(),
                self.oauth_server_url.clone(),
            )
            .await?;
            *lock = Token {
                current_access_token: new_access_token,
                access_token_expiration_time: expiration_time,
            };
        }

        Ok(lock.current_access_token.to_string())
    }

    async fn fetch_service_access_token(
        credentials: OAuthClientCredentials,
        issuer: String,
    ) -> Result<(String, SystemTime), AccessTokenError> {
        let token_url = format!("{issuer}/v1/token");

        let client = BasicClient::new(
            ClientId::new(credentials.client_id),
            Some(ClientSecret::new(credentials.client_secret)),
            AuthUrl::new(issuer)?,
            Some(TokenUrl::new(token_url)?),
        );

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new("api".to_string()))
            .request_async(async_http_client)
            .await
            .map_err(|err| AccessTokenError::Request(err.to_string()))?;

        let expiration_time = SystemTime::now().add(
            token_result
                .expires_in()
                .ok_or(AccessTokenError::TokenMissingExpiry)?,
        );

        Ok((
            token_result.access_token().secret().clone(),
            expiration_time,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    #[error("error when getting access token: {0}")]
    GetAccessToken(#[from] AccessTokenError),
}

#[async_trait::async_trait]
impl TokenRefresher for ClientConfiguration {
    type Error = RefreshError;

    async fn get_access_token(&self) -> Result<String, Self::Error> {
        self.internal_get_access_token(false)
            .await
            .map_err(Into::into)
    }

    async fn refresh_access_token(&self) -> Result<String, Self::Error> {
        self.internal_get_access_token(true)
            .await
            .map_err(Into::into)
    }

    #[cfg(feature = "otel-tracing")]
    fn base_url(&self) -> &str {
        unimplemented!()
    }

    #[cfg(feature = "otel-tracing")]
    fn tracing_configuration(&self) -> Option<&TracingConfiguration> {
        unimplemented!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccessTokenError {
    #[error("could not parse URL: {0}")]
    ParseUrl(#[from] oauth2::url::ParseError),
    #[error("request failed: {0}")]
    Request(String),
    #[error("token had no expiry date")]
    TokenMissingExpiry,
}
