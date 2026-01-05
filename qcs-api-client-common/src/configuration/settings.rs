//! Models and utilities for managing QCS settings.
use std::collections::HashMap;
use std::path::PathBuf;

use figment::providers::Format;
use figment::{providers::Toml, Figment};
use serde::{Deserialize, Serialize};

use crate::configuration::error::DiscoveryError;
use crate::configuration::oidc::{fetch_discovery, DISCOVERY_REQUIRED_SCOPE};
use crate::configuration::tokens::default_http_client;

use super::{
    env_or_default_quilc_url, env_or_default_qvm_url, expand_path_from_env_or_default, LoadError,
    DEFAULT_API_URL, DEFAULT_GRPC_API_URL, DEFAULT_PROFILE_NAME, DEFAULT_QUILC_URL,
    DEFAULT_QVM_URL,
};

/// Setting the `QCS_SETTINGS_FILE_PATH` environment variable will change which file is used for loading [`Settings`].
pub const SETTINGS_PATH_VAR: &str = "QCS_SETTINGS_FILE_PATH";
/// The default path that [`Settings`] will be loaded from;
pub const DEFAULT_SETTINGS_PATH: &str = "~/.qcs/settings.toml";

/// The structure of QCS settings, typically serialized as a TOML file at [`DEFAULT_SETTINGS_PATH`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Settings {
    /// The default profile to use - this should match a key of [`Settings::profiles`].
    #[serde(default = "default_profile_name")]
    pub default_profile_name: String,

    /// All named [`Profile`]s defined in the settings file.
    #[serde(default = "default_profiles")]
    pub profiles: HashMap<String, Profile>,

    /// All named [`AuthServer`]s defined in the settings file.
    #[serde(default = "default_auth_servers")]
    pub auth_servers: HashMap<String, AuthServer>,

    /// The path to the settings file this [`Settings`] was loaded from,
    /// if it was loaded from a file. This is not stored in the settings file itself.
    #[serde(skip)]
    pub file_path: Option<PathBuf>,
}

impl Settings {
    /// Load [`Settings`] from the path specified by the [`SETTINGS_PATH_VAR`] environment variable if set,
    /// or else the default path at [`DEFAULT_SETTINGS_PATH`].
    ///
    /// # Errors
    ///
    /// [`LoadError`] if the settings file cannot be loaded.
    pub fn load() -> Result<Self, LoadError> {
        let path = expand_path_from_env_or_default(SETTINGS_PATH_VAR, DEFAULT_SETTINGS_PATH)?;
        #[cfg(feature = "tracing")]
        tracing::debug!("loading QCS settings from {path:?}");
        Self::load_from_path(&path)
    }

    /// Load [`Settings`] from the path specified by `path`.
    ///
    /// # Errors
    ///
    /// [`LoadError`] if the settings file cannot be loaded.
    pub fn load_from_path(path: &PathBuf) -> Result<Self, LoadError> {
        let mut settings: Self = Figment::from(Toml::file(path)).extract()?;
        settings.file_path = Some(path.into());
        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_profile_name: default_profile_name(),
            profiles: default_profiles(),
            auth_servers: default_auth_servers(),
            file_path: None,
        }
    }
}

fn default_profile_name() -> String {
    DEFAULT_PROFILE_NAME.to_string()
}

fn default_profiles() -> HashMap<String, Profile> {
    HashMap::from([(DEFAULT_PROFILE_NAME.to_string(), Profile::default())])
}

fn default_auth_servers() -> HashMap<String, AuthServer> {
    HashMap::from([(DEFAULT_PROFILE_NAME.to_string(), AuthServer::default())])
}

/// A particular profile of [`Settings`], which defines all the configurable options
/// for connecting to a particular QCS instance using a particular set of credentials.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Profile {
    /// URL of the QCS REST API.
    #[serde(default = "default_api_url")]
    pub api_url: String,
    /// URL of the QCS gRPC API.
    #[serde(default = "default_grpc_api_url")]
    pub grpc_api_url: String,
    /// Name of the [`AuthServer`] to use.
    #[serde(default = "default_profile_name")]
    pub auth_server_name: String,
    /// Name of the [`Credential`][`super::secrets::Credential`] to use from the corresponding [`Secrets`][`super::secrets::Secrets`].
    #[serde(default = "default_profile_name")]
    pub credentials_name: String,
    /// Application specific settings.
    #[serde(default)]
    pub applications: Applications,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            grpc_api_url: DEFAULT_GRPC_API_URL.to_string(),
            auth_server_name: DEFAULT_PROFILE_NAME.to_string(),
            credentials_name: DEFAULT_PROFILE_NAME.to_string(),
            applications: Applications::default(),
        }
    }
}

fn default_api_url() -> String {
    DEFAULT_API_URL.to_string()
}

fn default_grpc_api_url() -> String {
    DEFAULT_GRPC_API_URL.to_string()
}

pub(crate) const QCS_DEFAULT_CLIENT_ID_PRODUCTION: &str = "0oa3ykoirzDKpkfzk357";
pub(crate) const QCS_DEFAULT_AUTH_ISSUER_PRODUCTION: &str =
    "https://auth.qcs.rigetti.com/oauth2/aus8jcovzG0gW2TUG355";

/// OAuth 2.0 authorization server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct AuthServer {
    /// OAuth 2.0 client id.
    pub client_id: String,
    /// OAuth 2.0 issuer URL.
    ///
    /// This is the base URL of the identity provider.
    /// For Okta, this usually looks like `https://example.okta.com/oauth2/default`.
    /// For Cognito, it might look like `https://cognito-idp.us-west-2.amazonaws.com/us-west-2_example`.
    ///
    /// Note that this is technically distinct from the `issuer` field in [`OidcDiscovery`],
    /// which is the canonical URI that the identity provider uses to sign and validate tokens,
    /// but the OpenID specification requires that they match exactly,
    /// and that they match the `iss` claim in Tokens issued by this identity provider.
    pub issuer: String,

    /// OAuth 2.0 scopes to request during authorization requests.
    /// If not specified, `supported_scopes` from the discovery document hosted at `issuer` will be used.
    /// The scope `openid` is always requested, even if not present in this list.
    pub scopes: Option<Vec<String>>,
}

impl Default for AuthServer {
    fn default() -> Self {
        Self {
            client_id: QCS_DEFAULT_CLIENT_ID_PRODUCTION.to_string(),
            issuer: QCS_DEFAULT_AUTH_ISSUER_PRODUCTION.to_string(),
            scopes: Some(vec![DISCOVERY_REQUIRED_SCOPE.to_string()]),
        }
    }
}

impl AuthServer {
    /// Create a new [`AuthServer`] with a `client_id` and `issuer` and an optional list of scopes.
    ///
    /// If `scopes` is [`None`], all `scopes_supported` from the issuer's discovery document will be used when requesting authorization tokens.
    /// Note that the required scope `openid` is always requested, even if `scopes` is provided but does not contain it.
    #[must_use]
    pub const fn new(client_id: String, issuer: String, scopes: Option<Vec<String>>) -> Self {
        Self {
            client_id,
            issuer,
            scopes,
        }
    }

    /// Create a new [`AuthServer`] with the specified `client_id` and `issuer`, populating `scopes` with all `scopes_supported` fetched from the issuer's discovery document.
    ///
    /// # Errors
    /// Returns an error if the discovery document cannot be fetched or parsed.
    pub async fn new_with_discovery_supported_scopes(
        client_id: String,
        issuer: String,
    ) -> Result<Self, DiscoveryError> {
        let client = default_http_client()?;
        let discovery = fetch_discovery(&client, &issuer).await?;
        Ok(Self {
            client_id,
            issuer,
            scopes: Some(discovery.scopes_supported),
        })
    }
}

/// Settings for secondary applications used by QCS SDKs.
#[derive(Deserialize, Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Applications {
    /// Settings for use of the pyquil SDK.
    #[serde(default)]
    pub pyquil: Pyquil,
}

/// Settings for secondary applications used by pyquil.
#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Pyquil {
    /// URL of the QVM server.
    #[serde(default = "env_or_default_qvm_url")]
    pub qvm_url: String,

    /// URL of the Quilc compiler server.
    #[serde(default = "env_or_default_quilc_url")]
    pub quilc_url: String,
}

impl Default for Pyquil {
    fn default() -> Self {
        Self {
            quilc_url: DEFAULT_QUILC_URL.to_string(),
            qvm_url: DEFAULT_QVM_URL.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{Settings, SETTINGS_PATH_VAR};

    #[test]
    fn returns_err_if_invalid_path_env() {
        figment::Jail::expect_with(|jail| {
            jail.set_env(SETTINGS_PATH_VAR, "/blah/doesnt_exist.toml");
            Settings::load().expect_err("Should return error when a file cannot be found.");
            Ok(())
        });
    }

    #[test]
    fn test_uses_defaults_incomplete_settings() {
        figment::Jail::expect_with(|jail| {
            let _ = jail.create_file("settings.toml", r#"default_profile_name = "TEST""#)?;
            jail.set_env(SETTINGS_PATH_VAR, "settings.toml");
            let loaded = Settings::load().expect("should load settings");
            let expected = Settings {
                default_profile_name: "TEST".to_string(),
                file_path: Some(PathBuf::from("settings.toml")),
                ..Settings::default()
            };

            assert_eq!(loaded, expected);

            Ok(())
        });
    }

    #[test]
    fn loads_from_env_var_path() {
        figment::Jail::expect_with(|jail| {
            let settings = Settings {
                default_profile_name: "TEST".to_string(),
                file_path: Some(PathBuf::from("secrets.toml")),
                ..Settings::default()
            };
            let settings_string =
                toml::to_string(&settings).expect("Should be able to serialize settings");

            _ = jail.create_file("secrets.toml", &settings_string)?;
            jail.set_env(SETTINGS_PATH_VAR, "secrets.toml");

            assert_eq!(settings, Settings::load().unwrap());

            Ok(())
        });
    }
}
