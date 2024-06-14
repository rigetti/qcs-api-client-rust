use std::collections::HashMap;

use figment::providers::Format;
use figment::{providers::Toml, Figment};
use serde::{Deserialize, Serialize};

use super::{
    expand_path_from_env_or_default, LoadError, DEFAULT_API_URL, DEFAULT_GRPC_API_URL,
    DEFAULT_PROFILE_NAME, DEFAULT_QUILC_URL, DEFAULT_QVM_URL,
};

/// Setting the `QCS_SETTINGS_FILE_PATH` environment variable will change which file is used for loading [`Settings`].
pub const SETTINGS_PATH_VAR: &str = "QCS_SETTINGS_FILE_PATH";
/// The default path that [`Settings`] will be loaded from;
pub const DEFAULT_SETTINGS_PATH: &str = "~/.qcs/settings.toml";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct Settings {
    #[serde(default = "default_profile_name")]
    pub(crate) default_profile_name: String,
    #[serde(default = "default_profiles")]
    pub(crate) profiles: HashMap<String, Profile>,
    #[serde(default = "default_auth_servers")]
    pub(crate) auth_servers: HashMap<String, AuthServer>,
}

impl Settings {
    pub(crate) fn load() -> Result<Self, LoadError> {
        let path = expand_path_from_env_or_default(SETTINGS_PATH_VAR, DEFAULT_SETTINGS_PATH)?;
        Ok(Figment::from(Toml::file(path)).extract()?)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_profile_name: default_profile_name(),
            profiles: default_profiles(),
            auth_servers: default_auth_servers(),
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub(crate) struct Profile {
    /// URL of the QCS REST API.
    #[serde(default = "default_api_url")]
    pub(crate) api_url: String,
    /// URL of the QCS gRPC API.
    #[serde(default = "default_grpc_api_url")]
    pub(crate) grpc_api_url: String,
    /// Name of the auth server to use.
    #[serde(default = "default_profile_name")]
    pub(crate) auth_server_name: String,
    /// Name of the credentials to use.
    #[serde(default = "default_profile_name")]
    pub(crate) credentials_name: String,
    /// Application specific settings.
    #[serde(default)]
    pub(crate) applications: Applications,
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

/// Okta authorization server.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[cfg_attr(feature = "python", pyo3::pyclass)]
pub struct AuthServer {
    /// Okta client id.
    client_id: String,
    /// Okta issuer URL.
    issuer: String,
}

impl Default for AuthServer {
    fn default() -> Self {
        Self {
            client_id: QCS_DEFAULT_CLIENT_ID_PRODUCTION.to_string(),
            issuer: QCS_DEFAULT_AUTH_ISSUER_PRODUCTION.to_string(),
        }
    }
}

impl AuthServer {
    /// Create a new [`AuthServer`] with a ``client_id`` and ``issuer``.
    #[must_use]
    pub const fn new(client_id: String, issuer: String) -> Self {
        Self { client_id, issuer }
    }

    /// Get the configured Okta client id.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Set an Okta client id.
    pub fn set_client_id(&mut self, id: String) {
        self.client_id = id;
    }

    /// Get the Okta issuer URL.
    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Set an Okta issuer URL.
    pub fn set_issuer(&mut self, issuer: String) {
        self.issuer = issuer;
    }
}

#[derive(Deserialize, Clone, Debug, Default, PartialEq, Serialize)]
pub(crate) struct Applications {
    pub(crate) pyquil: Pyquil,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Pyquil {
    pub(crate) quilc_url: String,
    pub(crate) qvm_url: String,
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
