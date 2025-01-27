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
//! * [`SECRETS_READ_ONLY_VAR`]: Flag indicating whether to treat the `secrets.toml` file as read-only. Disabled by default.
//!     * Access token updates will _not_ be persisted to the secrets file, regardless of file permissions, for any of the following values (case insensitive): "true", "yes", "1".  
//!     * Access token updates will be persisted to the secrets file if it is writeable for any other value or if unset.
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
use std::{env, path::PathBuf};

#[cfg(feature = "python")]
use pyo3::prelude::*;

use self::{
    secrets::{Credential, Secrets, TokenPayload},
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
pub use secrets::{DEFAULT_SECRETS_PATH, SECRETS_PATH_VAR, SECRETS_READ_ONLY_VAR};
pub use settings::{AuthServer, DEFAULT_SETTINGS_PATH, SETTINGS_PATH_VAR};
pub use tokens::{
    ClientCredentials, ExternallyManaged, OAuthGrant, OAuthSession, RefreshFunction, RefreshToken,
    TokenDispatcher, TokenRefresher,
};

const QCS_AUDIENCE: &str = "api://qcs";

/// Default profile name.
pub const DEFAULT_PROFILE_NAME: &str = "default";
/// Setting this environment variable will change which profile is used from the loaded config files
pub const PROFILE_NAME_VAR: &str = "QCS_PROFILE_NAME";
fn env_or_default_profile_name() -> String {
    env::var(PROFILE_NAME_VAR).unwrap_or_else(|_| DEFAULT_PROFILE_NAME.to_string())
}

/// Default URL to access the QCS API.
pub const DEFAULT_API_URL: &str = "https://api.qcs.rigetti.com";
/// Setting this environment variable will override the URL used to connect to the QCS REST API.
pub const API_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_API_URL";
fn env_or_default_api_url() -> String {
    env::var(API_URL_VAR).unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

/// Default URL to access the gRPC API.
pub const DEFAULT_GRPC_API_URL: &str = "https://grpc.qcs.rigetti.com";
/// Setting this environment variable will override the URL used to connect to the GRPC server.
pub const GRPC_API_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_GRPC_URL";
fn env_or_default_grpc_url() -> String {
    env::var(GRPC_API_URL_VAR).unwrap_or_else(|_| DEFAULT_GRPC_API_URL.to_string())
}

/// Default URL to access QVM.
pub const DEFAULT_QVM_URL: &str = "http://127.0.0.1:5000";
/// Setting this environment variable will override the URL used to access the QVM.
pub const QVM_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QVM_URL";
fn env_or_default_qvm_url() -> String {
    env::var(QVM_URL_VAR).unwrap_or_else(|_| DEFAULT_QVM_URL.to_string())
}

/// Default URL to access `quilc`.
pub const DEFAULT_QUILC_URL: &str = "tcp://127.0.0.1:5555";
/// Setting this environment variable will override the URL used to access quilc.
pub const QUILC_URL_VAR: &str = "QCS_SETTINGS_APPLICATIONS_QUILC_URL";
fn env_or_default_quilc_url() -> String {
    env::var(QUILC_URL_VAR).unwrap_or_else(|_| DEFAULT_QUILC_URL.to_string())
}

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
    #[builder(private, default = "env_or_default_profile_name()")]
    profile: String,

    #[doc = "The URL for the QCS REST API."]
    #[builder(default = "env_or_default_api_url()")]
    api_url: String,

    #[doc = "The URL for the QCS gRPC API."]
    #[builder(default = "env_or_default_grpc_url()")]
    grpc_api_url: String,

    #[doc = "The URL of the quilc server."]
    #[builder(default = "env_or_default_quilc_url()")]
    quilc_url: String,

    #[doc = "The URL of the QVM server."]
    #[builder(default = "env_or_default_qvm_url()")]
    qvm_url: String,

    /// Provides a single, semi-shared access to user credential tokens.
    ///
    /// Note that the tokens are *not* shared when the `ClientConfiguration` is created multiple
    /// times, e.g. through [`ClientConfiguration::load_default`].
    #[builder(default, setter(custom))]
    pub(crate) oauth_session: Option<TokenDispatcher>,

    #[builder(private, default = "ConfigSource::Builder")]
    source: ConfigSource,

    /// Configuration for tracing of network API calls. If `None`, tracing is disabled.
    #[cfg(feature = "tracing-config")]
    #[builder(default)]
    tracing_configuration: Option<TracingConfiguration>,
}

impl ClientConfigurationBuilder {
    /// The [`OAuthSession`] to use to authenticate with the QCS API.
    ///
    /// When set to [`None`], the configuration will not manage an OAuth Session, and access to the
    /// QCS API will be limited to unauthenticated routes.
    pub fn oauth_session(&mut self, oauth_session: Option<OAuthSession>) -> &mut Self {
        self.oauth_session = Some(oauth_session.map(Into::into));
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
            file_path: settings_path,
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

        let secrets_path = secrets.file_path;
        let credential = secrets.credentials.remove(&profile.credentials_name);
        let oauth_session = match credential {
            Some(Credential {
                token_payload:
                    Some(TokenPayload {
                        access_token,
                        refresh_token,
                        ..
                    }),
            }) => {
                Some(OAuthSession::new(
                    OAuthGrant::RefreshToken(RefreshToken::new(
                        // Some configurations do not populate or may use an
                        // empty string for the `refresh_token`, but are still
                        // valid sessions with a valid `access_token`.
                        //
                        // Because we found a `token_payload`, we must assume
                        // the user wants to construct an `OAuthSession`.
                        // Note that this is no guarantee of session validity.
                        refresh_token.unwrap_or_default(),
                    )),
                    auth_server,
                    access_token,
                ))
            }
            _ => None,
        };

        let api_url = env::var(API_URL_VAR).unwrap_or(profile.api_url);
        let quilc_url = env::var(QUILC_URL_VAR).unwrap_or(profile.applications.pyquil.quilc_url);
        let qvm_url = env::var(QVM_URL_VAR).unwrap_or(profile.applications.pyquil.qvm_url);
        let grpc_api_url = env::var(GRPC_API_URL_VAR).unwrap_or(profile.grpc_api_url);

        #[cfg(feature = "tracing-config")]
        let tracing_configuration =
            TracingConfiguration::from_env().map_err(LoadError::TracingFilterParseError)?;

        let source = match (settings_path, secrets_path) {
            (Some(settings_path), Some(secrets_path)) => ConfigSource::File {
                settings_path,
                secrets_path,
            },
            _ => ConfigSource::Default,
        };

        let mut builder = Self::builder();
        builder
            .oauth_session(oauth_session)
            .profile(profile_name)
            .source(source)
            .api_url(api_url)
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

    /// Get the [`TracingConfiguration`].
    #[cfg(feature = "tracing-config")]
    #[must_use]
    pub const fn tracing_configuration(&self) -> Option<&TracingConfiguration> {
        self.tracing_configuration.as_ref()
    }

    /// Get the source of the configuration.
    #[must_use]
    pub const fn source(&self) -> &ConfigSource {
        &self.source
    }

    /// Get a copy of the current [`OAuthSession`].
    ///
    /// Note: This is a _copy_, the contained tokens will become stale once they expire.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn oauth_session(&self) -> Result<OAuthSession, TokenError> {
        Ok(self
            .oauth_session
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
        let dispatcher = self
            .oauth_session
            .as_ref()
            .ok_or_else(|| TokenError::NoCredentials)?;
        match dispatcher.validate().await {
            Ok(tokens) => Ok(tokens),
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "tracing-config")]
                tracing::debug!("Refreshing access token because current one is invalid: {e}");
                dispatcher
                    .refresh(self.source(), self.profile())
                    .await
                    .map(|e| e.access_token().map(ToString::to_string))?
            }
        }
    }

    /// Refreshes the [`Tokens`] in use and returns the new bearer access token.
    ///
    /// # Errors
    ///
    /// See [`TokenError`]
    pub async fn refresh(&self) -> Result<OAuthSession, TokenError> {
        self.oauth_session
            .as_ref()
            .ok_or(TokenError::NoRefreshToken)?
            .refresh(self.source(), self.profile())
            .await
    }
}

/// Describes how a [`ClientConfiguration`] was initialized.
#[derive(Clone, Debug)]
pub enum ConfigSource {
    /// A [`ClientConfiguration`] derived from a [`ClientConfigurationBuilder`]
    Builder,
    /// A [`ClientConfiguration`] derived from at least one file.
    File {
        /// The path to the QCS `settings.toml` file used to initialize the [`ClientConfiguration`].
        settings_path: PathBuf,
        /// The path to a QCS `secrets.toml` file used to initialize the [`ClientConfiguration`].
        secrets_path: PathBuf,
    },
    /// A [`ClientConfiguration`] derived from default values.
    Default,
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
    use time::{Duration, OffsetDateTime};

    use crate::configuration::{
        expand_path_from_env_or_default, secrets::Secrets, settings::Settings, AuthServer,
        ClientConfiguration, OAuthSession, RefreshToken, API_URL_VAR, DEFAULT_QUILC_URL,
        GRPC_API_URL_VAR, QUILC_URL_VAR, QVM_URL_VAR,
    };

    use super::{
        settings::QCS_DEFAULT_AUTH_ISSUER_PRODUCTION, tokens::ClientCredentials, TokenRefresher,
        QCS_AUDIENCE,
    };

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

    #[tokio::test]
    async fn test_default_uses_env_var_overrides() {
        figment::Jail::expect_with(|jail| {
            let quilc_url = "quilc_url";
            let qvm_url = "qvm_url";
            let grpc_url = "grpc_url";
            let api_url = "api_url";

            jail.set_env(QUILC_URL_VAR, quilc_url);
            jail.set_env(QVM_URL_VAR, qvm_url);
            jail.set_env(GRPC_API_URL_VAR, grpc_url);
            jail.set_env(API_URL_VAR, api_url);

            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, quilc_url);
            assert_eq!(config.qvm_url, qvm_url);
            assert_eq!(config.grpc_api_url, grpc_url);
            assert_eq!(config.api_url, api_url);

            Ok(())
        });
    }

    #[test]
    fn test_default_loads_settings_with_partial_profile_applications() {
        figment::Jail::expect_with(|jail| {
            let directory = jail.directory();
            let settings_file_name = "settings.toml";
            let settings_file_path = directory.join(settings_file_name);

            let quilc_url_env_var = "env-var://quilc.url/after";

            let settings_file_contents = r#"
default_profile_name = "default"

[profiles]
[profiles.default]
api_url = ""
auth_server_name = "default"
credentials_name = "default"
applications = {}

[auth_servers]
[auth_servers.default]
client_id = ""
issuer = ""
"#;
            jail.create_file(settings_file_name, settings_file_contents)
                .expect("should create test settings.toml");

            jail.set_env(
                "QCS_SETTINGS_FILE_PATH",
                settings_file_path
                    .to_str()
                    .expect("settings file path should be a string"),
            );

            // before setting env var
            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, DEFAULT_QUILC_URL);

            jail.set_env("QCS_SETTINGS_APPLICATIONS_QUILC_URL", quilc_url_env_var);

            // after setting env var
            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, quilc_url_env_var);

            Ok(())
        });
    }

    #[test]
    fn test_default_loads_settings_with_partial_profile_applications_pyquil() {
        figment::Jail::expect_with(|jail| {
            let directory = jail.directory();
            let settings_file_name = "settings.toml";
            let settings_file_path = directory.join(settings_file_name);

            let quilc_url_settings_toml = "settings-toml://quilc.url";
            let quilc_url_env_var = "env-var://quilc.url/after";

            let settings_file_contents = format!(
                r#"
default_profile_name = "default"

[profiles]
[profiles.default]
api_url = ""
auth_server_name = "default"
credentials_name = "default"
applications.pyquil.quilc_url = "{quilc_url_settings_toml}"

[auth_servers]
[auth_servers.default]
client_id = ""
issuer = ""
"#
            );

            jail.create_file(settings_file_name, &settings_file_contents)
                .expect("should create test settings.toml");

            jail.set_env(
                "QCS_SETTINGS_FILE_PATH",
                settings_file_path
                    .to_str()
                    .expect("settings file path should be a string"),
            );

            // before setting env var
            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, quilc_url_settings_toml);

            jail.set_env("QCS_SETTINGS_APPLICATIONS_QUILC_URL", quilc_url_env_var);

            // after setting env var
            let config = ClientConfiguration::load_default().unwrap();
            assert_eq!(config.quilc_url, quilc_url_env_var);

            Ok(())
        });
    }

    #[tokio::test]
    async fn test_hydrate_access_token_on_load() {
        let mut config = ClientConfiguration::builder().build().unwrap();
        let access_token = "test_access_token";
        figment::Jail::expect_with(|jail| {
            let directory = jail.directory();
            let settings_file_name = "settings.toml";
            let settings_file_path = directory.join(settings_file_name);
            let secrets_file_name = "secrets.toml";
            let secrets_file_path = directory.join(secrets_file_name);

            let settings_file_contents = r#"
default_profile_name = "default"

[profiles]
[profiles.default]
api_url = ""
auth_server_name = "default"
credentials_name = "default"

[auth_servers]
[auth_servers.default]
client_id = ""
issuer = ""
"#;

            let secrets_file_contents = format!(
                r#"
[credentials]
[credentials.default]
[credentials.default.token_payload]
access_token = "{access_token}"
expires_in = 3600
id_token = "id_token"
refresh_token = "refresh_token"
scope = "offline_access openid profile email"
token_type = "Bearer"
"#
            );

            jail.create_file(settings_file_name, settings_file_contents)
                .expect("should create test settings.toml");
            jail.create_file(secrets_file_name, &secrets_file_contents)
                .expect("should create test settings.toml");

            jail.set_env(
                "QCS_SETTINGS_FILE_PATH",
                settings_file_path
                    .to_str()
                    .expect("settings file path should be a string"),
            );
            jail.set_env(
                "QCS_SECRETS_FILE_PATH",
                secrets_file_path
                    .to_str()
                    .expect("secrets file path should be a string"),
            );

            config = ClientConfiguration::load_default().unwrap();
            Ok(())
        });
        assert_eq!(
            config.get_access_token().await.unwrap(),
            Some(access_token.to_string())
        );
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
                exp: (OffsetDateTime::now_utc() + Duration::seconds(100)).unix_timestamp(),
                ..Self::default()
            }
        }

        fn new_expired() -> Self {
            Self {
                exp: (OffsetDateTime::now_utc() - Duration::seconds(100)).unix_timestamp(),
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
        let tokens = OAuthSession::from_refresh_token(
            RefreshToken::new(valid_token.clone()),
            AuthServer::default(),
            Some(valid_token.clone()),
        );
        assert_eq!(
            tokens
                .validate()
                .expect("Token should not fail validation."),
            valid_token
        );
    }

    #[test]
    fn test_expired_token() {
        let invalid_token = Claims::new_expired().to_encoded();
        let tokens = OAuthSession::from_refresh_token(
            RefreshToken::new(invalid_token),
            AuthServer::default(),
            None,
        );
        assert!(tokens.validate().is_err());
    }

    #[test]
    fn test_client_credentials_without_access_token() {
        let tokens = OAuthSession::from_client_credentials(
            ClientCredentials::new("client_id".to_string(), "client_secret".to_string()),
            AuthServer::default(),
            None,
        );
        assert!(tokens.validate().is_err());
    }

    #[tokio::test]
    async fn test_session_is_present_with_empty_refresh_token_and_valid_access_token() {
        let access_token = Claims::new_valid().to_encoded();
        let mut config = ClientConfiguration::builder().build().unwrap();
        figment::Jail::expect_with(|jail| {
            let directory = jail.directory();
            let settings_file_name = "settings.toml";
            let settings_file_path = directory.join(settings_file_name);
            let secrets_file_name = "secrets.toml";
            let secrets_file_path = directory.join(secrets_file_name);

            let settings_file_contents = r#"
default_profile_name = "default"

[profiles]
[profiles.default]
api_url = ""
auth_server_name = "default"
credentials_name = "default"

[auth_servers]
[auth_servers.default]
client_id = ""
issuer = ""
"#;

            // note this has no `refresh_token` property
            let secrets_file_contents = format!(
                r#"
[credentials]
[credentials.default]
[credentials.default.token_payload]
access_token = "{access_token}"
expires_in = 3600
id_token = "id_token"
scope = "offline_access openid profile email"
token_type = "Bearer"
"#
            );

            jail.create_file(settings_file_name, settings_file_contents)
                .expect("should create test settings.toml");
            jail.create_file(secrets_file_name, &secrets_file_contents)
                .expect("should create test secrets.toml");

            jail.set_env(
                "QCS_SETTINGS_FILE_PATH",
                settings_file_path
                    .to_str()
                    .expect("settings file path should be a string"),
            );
            jail.set_env(
                "QCS_SECRETS_FILE_PATH",
                secrets_file_path
                    .to_str()
                    .expect("secrets file path should be a string"),
            );

            config = ClientConfiguration::load_default().unwrap();
            Ok(())
        });

        assert_eq!(
            config.get_bearer_access_token().await.unwrap(),
            access_token.to_string()
        );
    }
}
