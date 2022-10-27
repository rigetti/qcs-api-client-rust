// Copyright 2022 Rigetti Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
/// Default URL to access the gRPC API.
pub const DEFAULT_GRPC_API_URL: &str = "https://legacy.grpc.qcs.rigetti.com";
/// Default URL to access QVM.
pub const DEFAULT_QVM_URL: &str = "http://127.0.0.1:5000";
/// Default URL to access `quilc`.
pub const DEFAULT_QUILC_URL: &str = "tcp://127.0.0.1:5555";

/// Setting this environment variable will override the URL used to access quilc.
pub const QUILC_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QUILC_URL";
/// Setting this environment variable will override the URL used to access the QVM.
pub const QVM_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QVM_URL";
/// Setting this environment variable will override the URL used to connect to the GRPC server.
pub const GRPC_API_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_GRPC_URL";

/// A single type containing an access token and an associated refresh token.
#[derive(Clone, Debug, Default)]
pub struct Tokens {
    /// The `Bearer` token to include in the `Authorization` header.
    pub bearer_access_token: Option<String>,
    /// The token used to refresh the access token.
    pub refresh_token: Option<String>,
}

/// All the config data that's parsed from config sources
#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ClientConfiguration {
    /// Provides a single, semi-shared access to user credential tokens.
    ///
    /// The use of `Arc` helps reduce excess token refreshes by sharing the
    /// tokens among all clones of the `ClientConfiguration`.
    ///
    /// Note that the tokens are *not* shared when the `ClientConfiguration` is created multiple
    /// times, e.g. through `load()`.
    tokens: Arc<Mutex<Tokens>>,

    /// Base URL of the QCS JSON HTTP API
    api_url: String,

    /// Information required for the refreshing of authentication tokens
    auth_server: AuthServer,

    /// Base URL of the QCS gRPC API
    grpc_api_url: String,

    quilc_url: String,
    qvm_url: String,
}

impl ClientConfiguration {
    /// URL to access the QCS API. Defaults to [`DEFAULT_API_URL`].
    #[must_use]
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// URL to access the gRPC API.
    #[must_use]
    pub fn grpc_api_url(&self) -> &str {
        &self.grpc_api_url
    }

    /// URL to access `quilc`. Defaults to [`DEFAULT_QUILC_URL`].
    #[must_use]
    pub fn quilc_url(&self) -> &str {
        &self.quilc_url
    }

    /// URL to access QVM. Defaults to [`DEFAULT_QVM_URL`].
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
    #[error("No refresh token is configured within selected QCS credential")]
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
    /// Attempt to load config files from `~/.qcs` and create a Configuration object
    /// for use with the QCS API.
    ///
    /// See <https://docs.rigetti.com/qcs/references/qcs-client-configuration> for details.
    ///
    /// # Errors
    ///
    /// See [`LoadError`].
    pub async fn load() -> Result<Self, LoadError> {
        let (settings, secrets) = try_join(settings::load(), secrets::load()).await?;
        Self::new(settings, secrets)
    }

    /// Manually set access and refresh tokens
    ///
    /// Most users do not want to use this. Instead, use [`ClientConfiguration::load()`], which uses your
    /// QCS configuration.
    pub async fn set_tokens(&mut self, tokens: Tokens) {
        let mut lock = self.tokens.lock().await;
        *lock = tokens;
    }

    fn validate_bearer_access_token(lock: &mut MutexGuard<Tokens>) -> Result<String, RefreshError> {
        match &lock.bearer_access_token {
            None => Err(RefreshError::NoRefreshToken),
            Some(token) => {
                let dummy_key = DecodingKey::from_secret(&[]);
                let mut validation = Validation::new(Algorithm::RS256);
                validation.validate_exp = true;
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

    /// Refresh the authentication tokens and return the new access token if successful.
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
            .timeout(std::time::Duration::from_secs(10))
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

        let quilc_url =
            std::env::var(QUILC_URL_VAR).unwrap_or(profile.applications.pyquil.quilc_url);
        let qvm_url = std::env::var(QVM_URL_VAR).unwrap_or(profile.applications.pyquil.qvm_url);
        let grpc_api_url = std::env::var(GRPC_API_URL_VAR).unwrap_or(profile.grpc_api_url);

        Ok(Self {
            api_url: profile.api_url,
            tokens: Arc::new(Mutex::new(Tokens {
                bearer_access_token: access_token,
                refresh_token,
            })),
            auth_server,
            quilc_url,
            qvm_url,
            grpc_api_url,
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
            api_url: DEFAULT_API_URL.to_string(),
            grpc_api_url: DEFAULT_GRPC_API_URL.to_string(),
            auth_server: AuthServer::default(),
            tokens: Arc::default(),
        }
    }
}

#[cfg(test)]
mod describe_client_configuration_load {
    use crate::configuration::*;

    #[tokio::test]
    async fn it_uses_env_var_overrides() {
        let quilc_url = "tcp://quilc:5555";
        let qvm_url = "http://qvm:5000";
        let grpc_url = "http://grpc:80";

        std::env::set_var(QUILC_URL_VAR, quilc_url);
        std::env::set_var(QVM_URL_VAR, qvm_url);
        std::env::set_var(GRPC_API_URL_VAR, grpc_url);

        let config = ClientConfiguration::new(Settings::default(), Secrets::default())
            .expect("config should load successfully");

        assert_eq!(config.quilc_url, quilc_url);
        assert_eq!(config.qvm_url, qvm_url);
        assert_eq!(config.grpc_api_url, grpc_url);
    }
}
