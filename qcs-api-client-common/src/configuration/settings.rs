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

use std::collections::HashMap;

use crate::configuration::DEFAULT_API_URL;
use serde::{Deserialize, Serialize};

use super::path::path_from_env_or_home;
use super::{LoadError, DEFAULT_GRPC_API_URL, DEFAULT_PROFILE_NAME};

/// Setting the `QCS_SETTINGS_FILE_PATH` environment variable will change which file is used for loading settings
pub const SETTINGS_PATH_VAR: &str = "QCS_SETTINGS_FILE_PATH";

pub(crate) async fn load() -> Result<Settings, LoadError> {
    let path = path_from_env_or_home(SETTINGS_PATH_VAR, "settings.toml")?;
    #[cfg(feature = "tracing")]
    tracing::debug!("attempting to load QCS settings from {}", path.display());
    let content =
        tokio::fs::read_to_string(&path)
            .await
            .map_err(|source| LoadError::FileOpenError {
                path: path.clone(),
                source,
            })?;
    toml::from_str(&content).map_err(|source| LoadError::FileParseError { path, source })
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub(crate) struct Settings {
    /// Which profile to select settings from when none is specified.
    #[serde(default = "default_profile_name")]
    pub(crate) default_profile_name: String,
    /// All available configuration profiles, keyed by profile name.
    #[serde(default = "default_profiles")]
    pub(crate) profiles: HashMap<String, Profile>,
    #[serde(default = "default_auth_servers")]
    pub(crate) auth_servers: HashMap<String, AuthServer>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_profile_name: "default".to_string(),
            profiles: default_profiles(),
            auth_servers: default_auth_servers(),
        }
    }
}

fn default_profiles() -> HashMap<String, Profile> {
    let mut map = HashMap::with_capacity(1);
    map.insert("default".to_string(), Profile::default());
    map
}

fn default_auth_servers() -> HashMap<String, AuthServer> {
    let mut map = HashMap::with_capacity(1);
    map.insert(DEFAULT_PROFILE_NAME.to_string(), AuthServer::default());
    map
}

fn default_profile_name() -> String {
    DEFAULT_PROFILE_NAME.to_string()
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub(crate) struct Profile {
    /// URL of the QCS API to use for all API calls
    pub(crate) api_url: String,
    #[serde(default = "default_grpc_api_url")]
    pub(crate) grpc_api_url: String,
    #[serde(default = "default_profile_name")]
    pub(crate) auth_server_name: String,
    #[serde(default = "default_profile_name")]
    pub(crate) credentials_name: String,
    #[serde(default)]
    pub(crate) applications: Applications,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            grpc_api_url: DEFAULT_GRPC_API_URL.to_string(),
            auth_server_name: "default".to_string(),
            credentials_name: "default".to_string(),
            applications: Applications::default(),
        }
    }
}

fn default_grpc_api_url() -> String {
    DEFAULT_GRPC_API_URL.to_string()
}

#[derive(Deserialize, Debug, Default, PartialEq, Serialize)]
pub(crate) struct Applications {
    pub(crate) pyquil: Pyquil,
}

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub(crate) struct Pyquil {
    pub(crate) qvm_url: String,
    pub(crate) quilc_url: String,
}

impl Default for Pyquil {
    fn default() -> Self {
        Self {
            qvm_url: super::DEFAULT_QVM_URL.to_string(),
            quilc_url: super::DEFAULT_QUILC_URL.to_string(),
        }
    }
}

/// Okta authorization server.
#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct AuthServer {
    /// Okta client id.
    client_id: String,
    /// Okta issuer URL.
    issuer: String,
}

#[allow(clippy::missing_const_for_fn)]
impl AuthServer {
    /// Get the configured Okta client id.
    #[must_use]
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Set an Okta client id.
    #[must_use]
    pub fn set_client_id(mut self, id: String) -> Self {
        self.client_id = id;
        self
    }

    /// Get the Okta issuer URL.
    #[must_use]
    pub fn issuer(&self) -> &str {
        &self.issuer
    }

    /// Set an Okta issuer URL.
    #[must_use]
    pub fn set_issuer(mut self, issuer: String) -> Self {
        self.issuer = issuer;
        self
    }
}

const QCS_DEFAULT_CLIENT_ID_PRODUCTION: &str = "0oa3ykoirzDKpkfzk357";
const QCS_DEFAULT_AUTH_ISSUER_PRODUCTION: &str =
    "https://auth.qcs.rigetti.com/oauth2/aus8jcovzG0gW2TUG355";

impl Default for AuthServer {
    fn default() -> Self {
        Self {
            client_id: QCS_DEFAULT_CLIENT_ID_PRODUCTION.to_string(),
            issuer: QCS_DEFAULT_AUTH_ISSUER_PRODUCTION.to_string(),
        }
    }
}

#[cfg(test)]
mod describe_load {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::{load, Settings, SETTINGS_PATH_VAR};

    #[tokio::test]
    async fn it_returns_default_if_missing_path() {
        std::env::set_var(SETTINGS_PATH_VAR, "/blah/doesnt_exist.toml");

        let settings = load().await;

        std::env::remove_var(SETTINGS_PATH_VAR);

        assert!(settings.is_err());
    }

    #[tokio::test]
    async fn it_uses_defaults_incomplete_settings() {
        let file = NamedTempFile::new().expect("should be able to create temporary file");
        std::env::set_var(SETTINGS_PATH_VAR, file.path());

        let loaded = load().await.expect("should load settings");

        assert_eq!(Settings::default(), loaded);
    }

    #[tokio::test]
    async fn it_loads_from_env_var_path() {
        let mut file = NamedTempFile::new().expect("should be able to create temporary file");
        let settings = Settings {
            default_profile_name: "THIS IS A TEST".to_string(),
            ..Settings::default()
        };
        let settings_string =
            toml::to_string(&settings).expect("should be able to serialize test settings");
        file.write_all(settings_string.as_bytes())
            .expect("Failed to write test settings");
        std::env::set_var(SETTINGS_PATH_VAR, file.path());

        let loaded = load().await.expect("should be able to load settings");

        assert_eq!(settings, loaded);
    }
}
