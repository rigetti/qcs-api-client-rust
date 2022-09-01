//! This module is used for loading configuration that will be used to connect either to real QPUs
//! (and supporting services) or the QVM.
//!
//! By default, all settings are loaded from files located
//! under your home directory in a `.qcs` folder. `settings.toml` will be used to load general
//! settings (e.g. which URLs to connect to) and `secrets.toml` will be used to load tokens for
//! authentication. Both "settings" and "secrets" files should contain profiles. The
//! `default_profile_name` in settings sets the profile to be used when there is no override. You
//! can set the [`PROFILE_NAME_VAR`] to select a different profile. You can also use
//! [`SECRETS_PATH_VAR`] and [`SETTINGS_PATH_VAR`] to change which files are loaded.

use std::path::PathBuf;
use std::sync::Arc;

use futures::future::try_join;
use jsonwebtoken::{decode, errors::Error as JWTError, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, MutexGuard};

use secrets::Secrets;
pub use secrets::SECRETS_PATH_VAR;
pub use settings::SETTINGS_PATH_VAR;
use settings::{AuthServer, Settings};

use crate::configuration::LoadError::AuthServerNotFound;

mod path;
mod secrets;
mod settings;

/// Default URL to access the QCS API.
pub const DEFAULT_API_URL: &str = "https://api.qcs.rigetti.com";
/// Default URL to access QVM.
pub const DEFAULT_QVM_URL: &str = "http://127.0.0.1:5000";
/// Default URL to access `quilc`.
pub const DEFAULT_QUILC_URL: &str = "tcp://127.0.0.1:5555";

#[derive(Clone, Debug, Default)]
struct Tokens {
    bearer_access_token: Option<String>,
    refresh_token: Option<String>,
}

/// All the config data that's parsed from config sources
#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ClientConfiguration {
    // Provides a single, semi-shared access to user credential tokens.
    //
    // `Arc` provides a reference-counted pointer, while `Mutex` helps prevent data races.
    //
    // This helps reduce excess token refreshes by sharing the (refreshed) tokens between all
    // clones of the `ClientConfiguration`.
    //
    // Note that the tokens are *not* shared when the `ClientConfiguration` is created multiple
    // times, e.g. through `load()`.
    tokens: Arc<Mutex<Tokens>>,
    api_url: String,
    auth_server: AuthServer,
    quilc_url: String,
    qvm_url: String,
}

impl ClientConfiguration {
    /// URL to access the QCS API. Defaults to [`DEFAULT_API_URL`].
    #[must_use]
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// URL to access `quilc` over TCP. Defaults to [`DEFAULT_QUILC_URL`].
    #[must_use]
    pub fn quilc_url(&self) -> &str {
        &self.quilc_url
    }

    /// URL to access QVM over HTTP. Defaults to [`DEFAULT_QVM_URL`].
    #[must_use]
    pub fn qvm_url(&self) -> &str {
        &self.qvm_url
    }
}

/// Setting this environment variable will change which profile is used from the loaded config files
pub const PROFILE_NAME_VAR: &str = "QCS_PROFILE_NAME";

/// Errors that may occur while refreshing the access token.
#[derive(Debug, thiserror::Error)]
pub enum RefreshError {
    /// No refresh token to do the refresh with.
    #[error("No refresh token is in secrets")]
    NoRefreshToken,
    /// Failed to fetch new token.
    #[error("Error fetching new token")]
    FetchError(#[from] reqwest::Error),
    /// Error occurred while validating token.
    #[error("Error validating existing token: {0}")]
    ValidationError(#[from] JWTError),
}

/// Errors that may occur while loading a `Configuration`.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// Configuration does not contain the expected profile.
    #[error("Expected profile {0} in settings.profiles but it didn't exist")]
    ProfileNotFound(String),
    /// Configuration does not contain the expected auth server name.
    #[error("Expected auth server {0} in settings.auth_servers but it didn't exist")]
    AuthServerNotFound(String),
    /// Could not determine user home directory.
    #[error("Failed to determine home directory. You can use an explicit path by setting the {env} environment variable")]
    HomeDirError {
        /// An environment variable that indicates the user home directory when set.
        env: String,
    },
    /// Failed to open configuration file.
    #[error("Could not open file at {path}: {source}")]
    FileOpenError {
        /// The file the could not be opened.
        path: PathBuf,
        /// The error from trying to open it.
        source: std::io::Error,
    },
    /// Failed to parse configuration file as TOML.
    #[error("Could not parse TOML file at {path}: {source}")]
    FileParseError {
        /// The file that failed to parse.
        path: PathBuf,
        /// The error from parsing.
        source: toml::de::Error,
    },
}

impl ClientConfiguration {
    /// Attempt to load config files from ~/.qcs and create a Configuration object
    /// for use with qcs-api.
    ///
    /// # Errors
    ///
    /// See [`LoadError`].
    pub async fn load() -> Result<Self, LoadError> {
        let (settings, secrets) = try_join(settings::load(), secrets::load()).await?;
        Self::new(settings, secrets)
    }

    fn validate_bearer_access_token(lock: &mut MutexGuard<Tokens>) -> Result<String, RefreshError> {
        match &lock.bearer_access_token {
            None => Err(RefreshError::NoRefreshToken),
            Some(token) => {
                let dummy_key = DecodingKey::from_secret(&[]);
                let mut validation = Validation::new(Algorithm::RS256);
                validation.validate_exp = true;
                validation.validate_nbf = true;
                validation.leeway = 0;
                validation.insecure_disable_signature_validation();
                decode::<toml::Value>(token, &dummy_key, &validation)
                    .map(|_| token.to_string())
                    .map_err(RefreshError::from)
            }
        }
    }

    /// Gets the `Bearer` access token, refreshing it if expired.
    ///
    /// # Errors
    ///
    /// See [`RefreshError`].
    pub async fn get_bearer_access_token(&self) -> Result<String, RefreshError> {
        let mut lock = self.tokens.lock().await;
        match Self::validate_bearer_access_token(&mut lock) {
            Ok(token) => Ok(token),
            Err(_) => self.internal_refresh(&mut lock).await,
        }
    }

    /// Refresh the `access_token` and return the new token if successful.
    ///
    /// # Errors
    ///
    /// See [`RefreshError`].
    pub async fn refresh(&self) -> Result<String, RefreshError> {
        let mut lock = self.tokens.lock().await;
        self.internal_refresh(&mut lock).await
    }

    async fn internal_refresh<'a>(
        &'a self,
        lock: &mut MutexGuard<'a, Tokens>,
    ) -> Result<String, RefreshError> {
        let refresh_token = lock
            .refresh_token
            .as_deref()
            .ok_or(RefreshError::NoRefreshToken)?;
        let token_url = format!("{}/v1/token", &self.auth_server.issuer);
        let data = TokenRequest::new(&self.auth_server.client_id, refresh_token);
        let resp = reqwest::Client::builder()
            .user_agent(format!(
                "QCS API Client (Rust)/{}",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?
            .post(token_url)
            .form(&data)
            .send()
            .await?;
        let response_data: TokenResponse = resp.error_for_status()?.json().await?;
        lock.bearer_access_token = Some(response_data.access_token.clone());
        lock.refresh_token = Some(response_data.refresh_token);
        Ok(response_data.access_token)
    }

    fn new(settings: Settings, mut secrets: Secrets) -> Result<Self, LoadError> {
        let Settings {
            default_profile_name,
            mut profiles,
            mut auth_servers,
        } = settings;
        let profile_name = std::env::var(PROFILE_NAME_VAR).unwrap_or(default_profile_name);
        let profile = profiles
            .remove(&profile_name)
            .ok_or(LoadError::ProfileNotFound(profile_name))?;
        let auth_server = auth_servers
            .remove(&profile.auth_server_name)
            .ok_or_else(|| AuthServerNotFound(profile.auth_server_name.clone()))?;

        let credential = secrets.credentials.remove(&profile.credentials_name);
        let (access_token, refresh_token) = match credential {
            Some(secrets::Credential {
                token_payload: Some(token_payload),
            }) => (token_payload.access_token, token_payload.refresh_token),
            _ => (None, None),
        };

        Ok(Self {
            api_url: profile.api_url,
            tokens: Arc::new(Mutex::new(Tokens {
                bearer_access_token: access_token,
                refresh_token,
            })),
            auth_server,
            quilc_url: profile.applications.pyquil.quilc_url,
            qvm_url: profile.applications.pyquil.qvm_url,
        })
    }
}

#[derive(Debug, Serialize)]
struct TokenRequest<'a> {
    grant_type: &'static str,
    client_id: &'a str,
    refresh_token: &'a str,
}

impl<'a> TokenRequest<'a> {
    const fn new(client_id: &'a str, refresh_token: &'a str) -> TokenRequest<'a> {
        Self {
            grant_type: "refresh_token",
            client_id,
            refresh_token,
        }
    }
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    refresh_token: String,
    access_token: String,
}

impl Default for ClientConfiguration {
    fn default() -> Self {
        Self {
            quilc_url: DEFAULT_QUILC_URL.to_string(),
            qvm_url: DEFAULT_QVM_URL.to_string(),
            api_url: DEFAULT_API_URL.to_owned(),
            auth_server: AuthServer::default(),
            tokens: Arc::default(),
        }
    }
}
