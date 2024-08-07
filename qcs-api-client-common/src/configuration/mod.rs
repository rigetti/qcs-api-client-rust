//! This module is used for loading configuration that will be used to connect either to real QPUs
//! (and supporting services) or the QVM.
//!
//! By default, all settings are loaded from files located under your home directory in the
//! `.qcs` folder. Within that folder:
//!
//! * `settings.toml` will be used to load general settings (e.g. which URLs to connect to).
//! * `secrets.toml` will be used to load tokens for authentication.
//!
//! Both files should contain profiles. Your settings should contain a `default_profile_name`
//! that determines which profile is loaded when no other profile is explicitly provided.
//!
//! If you don't have either of these files, see [the QCS credentials guide](https://docs.rigetti.com/qcs/guides/qcs-credentials) for details on how to obtain them.
//!
//! You can use environment variables to override values in your configuration:
//!
//! * [`SETTINGS_PATH_VAR`]: Set the path of the `settings.toml` file to load.
//! * [`SECRETS_PATH_VAR`]: Set the path of the `secrets.toml` file to load.
//! * [`PROFILE_NAME_VAR`]: Override the profile that is loaded by default
//! * [`QUILC_URL_VAR`]: Override the URL used for requests to the quilc server.
//! * [`QVM_URL_VAR`]: Override the URL used for requests to the QVM server.
//! * [`API_URL_VAR`]: Override the URL used for requests to the QCS REST API server.
//! * [`GRPC_API_URL_VAR`]: Override the URL used for requests to the QCS gRPC API.
//!
//! The [`ClientConfiguration`] exposes an API for loading and accessing your
//! configuration.

#[cfg(feature = "tracing-config")]
use crate::tracing_configuration::TracingConfiguration;
use derive_builder::Builder;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::{env, path::PathBuf};

#[cfg(feature = "python")]
use pyo3::prelude::*;

use self::{
    secrets::{Credential, Secrets},
    settings::Settings,
};

mod error;
#[cfg(feature = "python")]
mod py;
mod secrets;
mod settings;
mod tokens;

pub use error::{LoadError, TokenError};
#[cfg(feature = "python")]
pub(crate) use py::*;
pub use secrets::{DEFAULT_SECRETS_PATH, SECRETS_PATH_VAR};
pub use settings::{AuthServer, DEFAULT_SETTINGS_PATH, SETTINGS_PATH_VAR};
pub use tokens::{TokenDispatcher, TokenRefresher, Tokens};

/// Default URL to access the QCS API.
pub const DEFAULT_API_URL: &str = "https://api.qcs.rigetti.com";
/// Default URL to access the gRPC API.
pub const DEFAULT_GRPC_API_URL: &str = "https://grpc.qcs.rigetti.com";
/// Default URL to access QVM.
pub const DEFAULT_QVM_URL: &str = "http://127.0.0.1:5000";
/// Default URL to access `quilc`.
pub const DEFAULT_QUILC_URL: &str = "tcp://127.0.0.1:5555";
/// Default profile name.
pub const DEFAULT_PROFILE_NAME: &str = "default";
/// Setting this environment variable will change which profile is used from the loaded config files
pub const PROFILE_NAME_VAR: &str = "QCS_PROFILE_NAME";
/// Setting this environment variable will override the URL used to access quilc.
pub const QUILC_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QUILC_URL";
/// Setting this environment variable will override the URL used to access the QVM.
pub const QVM_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QVM_URL";
/// Setting this environment variable will override the URL used to connect to the GRPC server.
pub const API_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_API_URL";
/// Setting this environment variable will override the URL used to connect to the GRPC server.
pub const GRPC_API_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_GRPC_URL";

const QCS_AUDIENCE: &str = "api://qcs";

/// A configuration suitable for use as a QCS API Client.
///
/// This configuration can be constructed in a few ways.
///
/// The most common way is to use [`ClientConfiguration::load_default`]. This will load the
/// configuration associated with your default QCS profile.
///
/// When loading your config, any values set by environment variables will override the values in
/// your configuration files.
///
/// You can also build a configuration from scratch using [`ClientConfigurationBuilder`]. Using a
/// builder bypasses configuration files and environment overrides.
#[derive(Clone, Debug, Builder)]
#[cfg_attr(feature = "python", pyclass)]
pub struct ClientConfiguration {
    #[builder(private, default = "DEFAULT_PROFILE_NAME.to_string()")]
    profile: String,

    #[doc = "The URL for the QCS REST API."]
    #[builder(default = "DEFAULT_API_URL.to_string()")]
    api_url: String,

    #[doc = "The URL for the QCS gRPC API."]
    #[builder(default = "DEFAULT_GRPC_API_URL.to_string()")]
    grpc_api_url: String,

    #[doc = "The URL of the quilc server."]
    #[builder(default = "DEFAULT_QUILC_URL.to_string()")]
    quilc_url: String,

    #[doc = "The URL of the QVM server."]
    #[builder(default = "DEFAULT_QVM_URL.to_string()")]
    qvm_url: String,

    #[doc = "The Okta Authorization server."]
    #[builder(default)]
    auth_server: AuthServer,

    /// Provides a single, semi-shared access to user credential tokens.
    ///
    /// Note that the tokens are *not* shared when the `ClientConfiguration` is created multiple
    /// times, e.g. through [`ClientConfiguration::load_default`].
    #[builder(default, setter(custom))]
    pub(crate) tokens: Option<TokenDispatcher>,

    /// Configuration for tracing of network API calls. If `None`, tracing is disabled.
    #[cfg(feature = "tracing-config")]
    #[builder(default)]
    tracing_configuration: Option<TracingConfiguration>,
}

impl ClientConfigurationBuilder {
    /// Set the access and refresh tokens.
    pub fn tokens(&mut self, tokens: Option<Tokens>) -> &mut Self {
        self.tokens = Some(tokens.map(Into::into));
        self
    }
}

impl ClientConfiguration {
    fn new(
        settings: Settings,
        mut secrets: Secrets,
        profile_name: Option<String>,
    ) -> Result<Self, LoadError> {
        let Settings {
            default_profile_name,
            mut profiles,
            mut auth_servers,
        } = settings;
        let profile_name = profile_name
            .or_else(|| env::var(PROFILE_NAME_VAR).ok())
            .unwrap_or(default_profile_name);
        let profile = profiles
            .remove(&profile_name)
            .ok_or(LoadError::ProfileNotFound(profile_name.clone()))?;
        let auth_server = auth_servers
            .remove(&profile.auth_server_name)
            .ok_or_else(|| LoadError::AuthServerNotFound(profile.auth_server_name.clone()))?;

        let credential = secrets.credentials.remove(&profile.credentials_name);
        let (access_token, refresh_token) = match credential {
            Some(Credential {
                token_payload: Some(token_payload),
            }) => (token_payload.access_token, token_payload.refresh_token),
            _ => (None, None),
        };

        let quilc_url = env::var(QUILC_URL_VAR).unwrap_or(profile.applications.pyquil.quilc_url);
        let qvm_url = env::var(QVM_URL_VAR).unwrap_or(profile.applications.pyquil.qvm_url);
        let grpc_api_url = env::var(GRPC_API_URL_VAR).unwrap_or(profile.grpc_api_url);

        let tokens = if let (Some(bearer_access_token), Some(refresh_token)) =
            (access_token, refresh_token)
        {
            Some(Tokens {
                bearer_access_token,
                refresh_token,
                auth_server: auth_server.clone(),
            })
        } else {
            None
        };

        #[cfg(feature = "tracing-config")]
        let tracing_configuration =
            TracingConfiguration::from_env().map_err(LoadError::TracingFilterParseError)?;

        let mut builder = Self::builder();
        builder
            .profile(profile_name)
            .tokens(tokens)
            .auth_server(auth_server)
            .api_url(profile.api_url)
            .quilc_url(quilc_url)
            .qvm_url(qvm_url)
            .grpc_api_url(grpc_api_url);

        #[cfg(feature = "tracing-config")]
        {
            builder.tracing_configuration(tracing_configuration);
        };

        Ok({
            builder
                .build()
                .expect("curated build process should not fail")
        })
    }

    /// Attempts to load config files
    ///
    /// # Errors
    ///
    /// See [`LoadError`]
    pub fn load_default() -> Result<Self, LoadError> {
        let base_config = Self::load(None)?;
        Ok(base_config)
    }

    /// Attempts to load a QCS configuration and creates a [`ClientConfiguration`] using the
    /// specified profile.
    ///
    /// # Errors
    ///
    /// See [`LoadError`]
    pub fn load_profile(profile_name: String) -> Result<Self, LoadError> {
        Self::load(Some(profile_name))
    }

    /// Attempts to load a QCS configuration and creates a [`ClientConfiguration`] using the
    /// specified profile. If no `profile_name` is provided, then a default configuration is
    /// loaded.
    ///
    /// # Errors
    ///
    /// See [`LoadError`]
    fn load(profile_name: Option<String>) -> Result<Self, LoadError> {
        #[cfg(feature = "tracing-config")]
        match profile_name.as_ref() {
            None => tracing::debug!("loading default QCS profile"),
            Some(profile) => tracing::debug!("loading QCS profile {profile}"),
        }
        let settings = Settings::load()?;
        let secrets = Secrets::load()?;

        Self::new(settings, secrets, profile_name)
    }

    /// Get a [`ClientConfigurationBuilder`]
    #[must_use]
    pub fn builder() -> ClientConfigurationBuilder {
        ClientConfigurationBuilder::default()
    }

    /// Get the name of the profile that was loaded, if any.
    #[must_use]
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Get the URL of the QCS REST API.
    #[must_use]
    pub fn api_url(&self) -> &str {
        &self.api_url
    }

    /// Get the URL of the QCS gRPC API.
    #[must_use]
    pub fn grpc_api_url(&self) -> &str {
        &self.grpc_api_url
    }

    /// Get the URL of the quilc server.
    #[must_use]
    pub fn quilc_url(&self) -> &str {
        &self.quilc_url
    }

    /// Get the URL of the QVM server.
    #[must_use]
    pub fn qvm_url(&self) -> &str {
        &self.qvm_url
    }

    /// Get the [`AuthServer`].
    #[must_use]
    pub const fn auth_server(&self) -> &AuthServer {
        &self.auth_server
    }

    /// Get the [`TracingConfiguration`].
    #[cfg(feature = "tracing-config")]
    #[must_use]
    pub const fn tracing_configuration(&self) -> Option<&TracingConfiguration> {
        self.tracing_configuration.as_ref()
    }

    /// Get a copy of the current [`Tokens`] in use.
    ///
    /// Note: This is a _copy_, the tokens will become stale when the tokens are refreshed.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn tokens(&self) -> Result<Tokens, TokenError> {
        Ok(self
            .tokens
            .as_ref()
            .ok_or(TokenError::NoRefreshToken)?
            .tokens()
            .await)
    }

    /// Gets the `Bearer` access token, refreshing it if it is expired.
    ///
    /// # Errors
    ///
    /// See [`TokenError`].
    pub async fn get_bearer_access_token(&self) -> Result<String, TokenError> {
        let tokens = self.tokens().await?;
        let validation = Self::validated_bearer_access_token(&tokens);
        match validation {
            Some(token) => Ok(token),
            None => self
                .refresh()
                .await
                .map(|tokens| tokens.bearer_access_token),
        }
    }

    fn validated_bearer_access_token(tokens: &Tokens) -> Option<String> {
        let token = &tokens.bearer_access_token;
        let placeholder_key = DecodingKey::from_secret(&[]);
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        validation.leeway = 60;
        validation.set_audience(&[QCS_AUDIENCE]);
        validation.insecure_disable_signature_validation();
        decode::<toml::Value>(token, &placeholder_key, &validation)
            .map(|_| token.clone())
            .ok()
    }

    /// Refreshes the [`Tokens`] in use and returns the new bearer access token.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn refresh(&self) -> Result<Tokens, TokenError> {
        self.tokens
            .as_ref()
            .ok_or(TokenError::NoRefreshToken)?
            .refresh()
            .await
    }
}

fn expand_path_from_env_or_default(
    env_var_name: &str,
    default: &str,
) -> Result<PathBuf, LoadError> {
    match env::var(env_var_name) {
        Ok(path) => {
            let expanded_path = shellexpand::env(&path).map_err(LoadError::from)?;
            let path_buf: PathBuf = expanded_path.as_ref().into();
            if !path_buf.exists() {
                return Err(LoadError::Path {
                    path: path_buf,
                    message: format!("The given path does not exist: {path}"),
                });
            }
            Ok(path_buf)
        }
        Err(env::VarError::NotPresent) => {
            let expanded_path = shellexpand::tilde(default);
            let path_buf: PathBuf = expanded_path.as_ref().into();
            if !path_buf.exists() {
                return Err(LoadError::Path {
                    path: path_buf,
                    message: format!(
                        "Could not find a QCS configuration at the default path: {default}"
                    ),
                });
            }
            Ok(path_buf)
        }
        Err(other_error) => Err(LoadError::EnvVar {
            variable_name: env_var_name.to_string(),
            message: other_error.to_string(),
        }),
    }
}

#[cfg(test)]
mod test {

    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;

    use crate::configuration::{
        expand_path_from_env_or_default, secrets::Secrets, settings::Settings, AuthServer,
        ClientConfiguration, Tokens, API_URL_VAR, GRPC_API_URL_VAR, QUILC_URL_VAR, QVM_URL_VAR,
    };

    use super::{settings::QCS_DEFAULT_AUTH_ISSUER_PRODUCTION, QCS_AUDIENCE};

    #[test]
    fn expands_env_var() {
        figment::Jail::expect_with(|jail| {
            let dir = jail.create_dir("~/blah/blah/")?;
            jail.create_file(dir.join("file.toml"), "")?;
            jail.set_env("SOME_PATH", "blah/blah");
            jail.set_env("SOME_VAR", "~/$SOME_PATH/file.toml");
            let secrets_path = expand_path_from_env_or_default("SOME_VAR", "default").unwrap();
            assert_eq!(secrets_path.to_str().unwrap(), "~/blah/blah/file.toml");

            Ok(())
        });
    }

    #[test]
    fn uses_env_var_overrides() {
        figment::Jail::expect_with(|jail| {
            let quilc_url = "tcp://quilc:5555";
            let qvm_url = "http://qvm:5000";
            let grpc_url = "http://grpc:80";
            let api_url = "http://api:80";

            jail.set_env(QUILC_URL_VAR, quilc_url);
            jail.set_env(QVM_URL_VAR, qvm_url);
            jail.set_env(API_URL_VAR, api_url);
            jail.set_env(GRPC_API_URL_VAR, grpc_url);

            let config = ClientConfiguration::new(
                Settings::default(),
                Secrets::default(),
                Some("default".to_string()),
            )
            .expect("Should be able to build default config.");

            assert_eq!(config.quilc_url, quilc_url);
            assert_eq!(config.qvm_url, qvm_url);
            assert_eq!(config.grpc_api_url, grpc_url);

            Ok(())
        });
    }

    #[test]
    fn test_default_uses_env_var_overrides() {
        figment::Jail::expect_with(|jail| {
            let quilc_url = "quilc_url";
            let qvm_url = "qvm_url";
            let grpc_url = "grpc_url";

            jail.set_env(QUILC_URL_VAR, quilc_url);
            jail.set_env(QVM_URL_VAR, qvm_url);
            jail.set_env(GRPC_API_URL_VAR, grpc_url);

            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, quilc_url);
            assert_eq!(config.qvm_url, qvm_url);
            assert_eq!(config.grpc_api_url, grpc_url);

            Ok(())
        });
    }

    #[derive(Clone, Debug, Serialize)]
    struct Claims {
        exp: i64,
        aud: String,
        iss: String,
        sub: String,
    }

    impl Default for Claims {
        fn default() -> Self {
            Self {
                exp: 0,
                aud: QCS_AUDIENCE.to_string(),
                iss: QCS_DEFAULT_AUTH_ISSUER_PRODUCTION.to_string(),
                sub: "qcs@rigetti.com".to_string(),
            }
        }
    }

    impl Claims {
        fn new_valid() -> Self {
            Self {
                exp: (chrono::Utc::now() + chrono::Duration::seconds(100)).timestamp(),
                ..Self::default()
            }
        }

        fn new_expired() -> Self {
            Self {
                exp: (chrono::Utc::now() - chrono::Duration::seconds(100)).timestamp(),
                ..Self::default()
            }
        }

        fn to_encoded(&self) -> String {
            encode(&Header::default(), &self, &EncodingKey::from_secret(&[])).unwrap()
        }
    }

    #[test]
    fn test_valid_token() {
        let valid_token = Claims::new_valid().to_encoded();
        let tokens = Tokens {
            bearer_access_token: valid_token.clone(),
            refresh_token: "refresh".to_string(),
            auth_server: AuthServer::default(),
        };
        assert_eq!(
            ClientConfiguration::validated_bearer_access_token(&tokens),
            Some(valid_token)
        );
    }

    #[test]
    fn test_expired_token() {
        let invalid_token = Claims::new_expired().to_encoded();
        let tokens = Tokens {
            bearer_access_token: invalid_token,
            refresh_token: "refresh".to_string(),
            auth_server: AuthServer::default(),
        };
        assert_eq!(
            ClientConfiguration::validated_bearer_access_token(&tokens),
            None
        );
    }
}
