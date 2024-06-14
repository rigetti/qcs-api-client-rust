use std::collections::HashMap;

use figment::providers::{Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};

use crate::configuration::LoadError;

use super::{expand_path_from_env_or_default, DEFAULT_PROFILE_NAME};

/// Setting the `QCS_SECRETS_FILE_PATH` environment variable will change which file is used for loading secrets
pub const SECRETS_PATH_VAR: &str = "QCS_SECRETS_FILE_PATH";
/// The default path that [`Secrets`] will be loaded from;
pub const DEFAULT_SECRETS_PATH: &str = "~/.qcs/secrets.toml";

#[derive(Deserialize, Debug, PartialEq, Serialize)]
pub(crate) struct Secrets {
    #[serde(default = "default_credentials")]
    pub(crate) credentials: HashMap<String, Credential>,
}

fn default_credentials() -> HashMap<String, Credential> {
    HashMap::from([(DEFAULT_PROFILE_NAME.to_string(), Credential::default())])
}

impl Default for Secrets {
    fn default() -> Self {
        Self {
            credentials: default_credentials(),
        }
    }
}

impl Secrets {
    pub(crate) fn load() -> Result<Self, LoadError> {
        let path = expand_path_from_env_or_default(SECRETS_PATH_VAR, DEFAULT_SECRETS_PATH)?;
        Ok(Figment::from(Toml::file(path)).extract()?)
    }
}

#[derive(Deserialize, Debug, Default, PartialEq, Serialize)]
pub(crate) struct Credential {
    pub(crate) token_payload: Option<TokenPayload>,
}

#[derive(Deserialize, Debug, Default, PartialEq, Serialize)]
pub(crate) struct TokenPayload {
    pub(crate) refresh_token: Option<String>,
    pub(crate) access_token: Option<String>,
    scope: Option<String>,
    expires_in: Option<u32>,
    id_token: Option<String>,
    token_type: Option<String>,
}

#[cfg(test)]
mod describe_load {
    use super::{Credential, Secrets, SECRETS_PATH_VAR};

    #[test]
    fn returns_err_if_invalid_path_env() {
        figment::Jail::expect_with(|jail| {
            jail.set_env(SECRETS_PATH_VAR, "/blah/doesnt_exist.toml");
            Secrets::load().expect_err("Should return error when a file cannot be found.");
            Ok(())
        });
    }

    #[test]
    fn loads_from_env_var_path() {
        figment::Jail::expect_with(|jail| {
            let mut secrets = Secrets::default();
            secrets
                .credentials
                .insert("test".to_string(), Credential::default());
            let secrets_string =
                toml::to_string(&secrets).expect("Should be able to serialize settings");

            _ = jail.create_file("secrets.toml", &secrets_string)?;
            jail.set_env(SECRETS_PATH_VAR, "secrets.toml");

            assert_eq!(secrets, Secrets::load().unwrap());

            Ok(())
        });
    }
}
