//! Models and utilities for managing QCS secret credentials.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_tempfile::TempFile;
use figment::providers::{Format, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, PrimitiveDateTime};
use tokio::io::AsyncWriteExt;
use toml_edit::{DocumentMut, Item};

use crate::configuration::LoadError;

use super::error::{IoErrorWithPath, IoOperation, WriteError};
use super::{expand_path_from_env_or_default, DEFAULT_PROFILE_NAME};

pub use super::secret_string::{SecretAccessToken, SecretRefreshToken};

/// Setting the `QCS_SECRETS_FILE_PATH` environment variable will change which file is used for loading secrets
pub const SECRETS_PATH_VAR: &str = "QCS_SECRETS_FILE_PATH";
/// `QCS_SECRETS_READ_ONLY` indicates whether to treat the `secrets.toml` file as read-only. Disabled by default.
/// * Access token updates will _not_ be persisted to the secrets file, regardless of file permissions, for any of the following values (case insensitive): "true", "yes", "1".  
/// * Access token updates will be persisted to the secrets file if it is writeable for any other value or if unset.
pub const SECRETS_READ_ONLY_VAR: &str = "QCS_SECRETS_READ_ONLY";
/// The default path that [`Secrets`] will be loaded from
pub const DEFAULT_SECRETS_PATH: &str = "~/.qcs/secrets.toml";

/// The structure of QCS secrets, typically serialized as a TOML file at [`DEFAULT_SECRETS_PATH`].
#[derive(Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct Secrets {
    /// All named [`Credential`]s defined in the secrets file.
    #[serde(default = "default_credentials")]
    pub credentials: HashMap<String, Credential>,
    /// The path to the secrets file this [`Secrets`] was loaded from,
    /// if it was loaded from a file. This is not stored in the secrets file itself.
    #[serde(skip)]
    pub file_path: Option<PathBuf>,
}

fn default_credentials() -> HashMap<String, Credential> {
    HashMap::from([(DEFAULT_PROFILE_NAME.to_string(), Credential::default())])
}

impl Default for Secrets {
    fn default() -> Self {
        Self {
            credentials: default_credentials(),
            file_path: None,
        }
    }
}

impl Secrets {
    /// Load [`Secrets`] from the path specified by the [`SECRETS_PATH_VAR`] environment variable if set,
    /// or else the default path at [`DEFAULT_SECRETS_PATH`].
    ///
    /// # Errors
    ///
    /// [`LoadError`] if the secrets file cannot be loaded.
    pub fn load() -> Result<Self, LoadError> {
        let path = expand_path_from_env_or_default(SECRETS_PATH_VAR, DEFAULT_SECRETS_PATH)?;
        #[cfg(feature = "tracing")]
        tracing::debug!("loading QCS secrets from {path:?}");
        Self::load_from_path(&path)
    }

    /// Load [`Secrets`] from the path specified by `path`.
    ///
    /// # Errors
    ///
    /// [`LoadError`] if the secrets file cannot be loaded.
    pub fn load_from_path(path: &PathBuf) -> Result<Self, LoadError> {
        let mut secrets: Self = Figment::from(Toml::file(path)).extract()?;
        secrets.file_path = Some(path.into());
        Ok(secrets)
    }

    /// Returns a bool indicating whether or not the QCS [`Secrets`] file is read-only.
    ///
    /// The file is considered read-only if the [`SECRETS_READ_ONLY_VAR`] environment variable is set,
    /// or if the file permissions indicate that it is read-only.
    ///
    /// # Errors
    ///
    /// [`WriteError`] if the file permissions cannot be checked.
    pub async fn is_read_only(
        secrets_path: impl AsRef<Path> + Send + Sync,
    ) -> Result<bool, WriteError> {
        // Check if the QCS_SECRETS_READ_ONLY environment variable is set
        let ro_env = std::env::var(SECRETS_READ_ONLY_VAR);
        let ro_env_lowercase = ro_env.as_deref().map(str::to_lowercase);
        if let Ok("true" | "yes" | "1") = ro_env_lowercase.as_deref() {
            return Ok(true);
        }

        // Check file permissions
        let is_read_only = tokio::fs::metadata(&secrets_path)
            .await
            .map_err(|error| IoErrorWithPath {
                error,
                path: secrets_path.as_ref().to_path_buf(),
                operation: IoOperation::GetMetadata,
            })?
            .permissions()
            .readonly();
        Ok(is_read_only)
    }

    /// Attempts to write a refresh and access token to the QCS [`Secrets`] file at
    /// the given path.
    ///
    /// The access token will only be updated if the access token currently stored in the file is
    /// older than the provided `updated_at` timestamp.
    ///
    /// # Errors
    ///
    /// - [`TokenError`] for possible errors.
    pub(crate) async fn write_tokens(
        secrets_path: impl AsRef<Path> + Send + Sync + std::fmt::Debug,
        profile_name: &str,
        refresh_token: Option<&SecretRefreshToken>,
        access_token: &SecretAccessToken,
        updated_at: OffsetDateTime,
    ) -> Result<(), WriteError> {
        // Read the current contents of the secrets file
        let secrets_string = tokio::fs::read_to_string(&secrets_path)
            .await
            .map_err(|error| IoErrorWithPath {
                error,
                path: secrets_path.as_ref().to_path_buf(),
                operation: IoOperation::Read,
            })?;

        // Parse the TOML content into a mutable document
        let mut secrets_toml = secrets_string.parse::<DocumentMut>()?;

        // Navigate to the `[credentials.<profile_name>.token_payload]` table
        let token_payload = Self::get_token_payload_table(&mut secrets_toml, profile_name)?;

        let current_updated_at = token_payload
            .get("updated_at")
            .and_then(|v| v.as_str())
            .and_then(|s| PrimitiveDateTime::parse(s, &Rfc3339).ok())
            .map(PrimitiveDateTime::assume_utc);

        let did_update_access_token = if current_updated_at.is_none_or(|dt| dt < updated_at) {
            token_payload["access_token"] = access_token.secret().into();
            token_payload["updated_at"] = updated_at.format(&Rfc3339)?.into();
            true
        } else {
            false
        };

        let did_update_refresh_token = refresh_token.is_some_and(|new_refresh_token| {
            let current_refresh_token = token_payload.get("refresh_token").and_then(|v| v.as_str());
            let new_refresh_token = new_refresh_token.secret();

            let is_changed = current_refresh_token != Some(new_refresh_token);
            if is_changed {
                token_payload["refresh_token"] = new_refresh_token.into();
            }
            is_changed
        });

        if did_update_access_token || did_update_refresh_token {
            // Create a temporary file
            // Write the updated TOML content to a temporary file.
            // The file is named using a newly generated UUIDv4 to avoid collisions
            // with other processes that may also be attempting to update the secrets file.
            let mut temp_file = TempFile::new().await?;
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "Created temporary QCS secrets file at {:?}",
                temp_file.file_path()
            );
            // Set the same permissions as the original file
            let secrets_file_permissions = tokio::fs::metadata(&secrets_path)
                .await
                .map_err(|error| IoErrorWithPath {
                    error,
                    path: secrets_path.as_ref().to_path_buf(),
                    operation: IoOperation::GetMetadata,
                })?
                .permissions();
            temp_file
                .set_permissions(secrets_file_permissions)
                .await
                .map_err(|error| IoErrorWithPath {
                    error,
                    path: temp_file.file_path().clone(),
                    operation: IoOperation::SetPermissions,
                })?;

            // Write the updated TOML content to the temporary file
            temp_file
                .write_all(secrets_toml.to_string().as_bytes())
                .await
                .map_err(|error| IoErrorWithPath {
                    error,
                    path: temp_file.file_path().clone(),
                    operation: IoOperation::Write,
                })?;
            temp_file.flush().await.map_err(|error| IoErrorWithPath {
                error,
                path: temp_file.file_path().clone(),
                operation: IoOperation::Flush,
            })?;

            // Atomically replace the original file with the temporary file.
            // Note that this will fail if the secrets file is on a different mount-point from `std::env::temp_dir()`.
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "Overwriting QCS secrets file at {secrets_path:?} with temporary file at {:?}",
                temp_file.file_path()
            );
            tokio::fs::rename(temp_file.file_path(), &secrets_path)
                .await
                .map_err(|error| IoErrorWithPath {
                    error,
                    path: temp_file.file_path().clone(),
                    operation: IoOperation::Rename {
                        dest: secrets_path.as_ref().to_path_buf(),
                    },
                })?;
        }

        Ok(())
    }

    /// Get the `[credentials.<profile_name>.token_payload]` table from the TOML document
    fn get_token_payload_table<'a>(
        secrets_toml: &'a mut DocumentMut,
        profile_name: &str,
    ) -> Result<&'a mut Item, WriteError> {
        secrets_toml
            .get_mut("credentials")
            .and_then(|credentials| credentials.get_mut(profile_name)?.get_mut("token_payload"))
            .ok_or_else(|| {
                WriteError::MissingTable(format!("credentials.{profile_name}.token_payload",))
            })
    }
}

/// A QCS credential, containing sensitive authentication secrets.
#[derive(Deserialize, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Credential {
    /// The [`TokenPayload`] for this credential.
    pub token_payload: Option<TokenPayload>,
}

/// A QCS token payload, containing sensitive authentication secrets.
#[derive(Deserialize, Debug, Default, PartialEq, Eq, Serialize)]
pub struct TokenPayload {
    /// The refresh token for this credential.
    pub refresh_token: Option<SecretRefreshToken>,
    /// The access token for this credential.
    pub access_token: Option<SecretAccessToken>,
    /// The time at which this token was last updated.
    #[serde(
        default,
        deserialize_with = "time::serde::rfc3339::option::deserialize",
        serialize_with = "time::serde::rfc3339::option::serialize"
    )]
    pub updated_at: Option<OffsetDateTime>,

    // The below fields are retained for (de)serialization for compatibility with other
    // libraries that use token payloads, but are not relevant here.
    scope: Option<String>,
    expires_in: Option<u32>,
    id_token: Option<String>,
    token_type: Option<String>,
}

#[cfg(test)]
mod describe_load {
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    use time::{macros::datetime, OffsetDateTime};

    use crate::configuration::secrets::SecretAccessToken;

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
            let mut secrets = Secrets {
                file_path: Some(PathBuf::from("env_secrets.toml")),
                ..Secrets::default()
            };
            secrets
                .credentials
                .insert("test".to_string(), Credential::default());
            let secrets_string =
                toml::to_string(&secrets).expect("Should be able to serialize secrets");

            _ = jail.create_file("env_secrets.toml", &secrets_string)?;
            jail.set_env(SECRETS_PATH_VAR, "env_secrets.toml");

            assert_eq!(secrets, Secrets::load().unwrap());

            Ok(())
        });
    }

    const fn max_rfc3339() -> OffsetDateTime {
        // PrimitiveDateTime::MAX can be larger than what can fit in a RFC3339 timestamp if the `time` crate's `large-dates` feature is enabled.
        // Instead of asserting that the `time` crate's `large-dates` feature is disabled, we use a hardcoded max value here.
        datetime!(9999-12-31 23:59:59.999_999_999).assume_utc()
    }

    #[test]
    fn test_write_access_token() {
        figment::Jail::expect_with(|jail| {
            let secrets_file_contents = r#"
[credentials]
[credentials.test]
[credentials.test.token_payload]
access_token = "old_access_token"
expires_in = 3600
id_token = "id_token"
refresh_token = "refresh_token"
scope = "offline_access openid profile email"
token_type = "Bearer"
"#;

            jail.create_file("secrets.toml", secrets_file_contents)
                .expect("should create test secrets.toml");
            let mut original_permissions = std::fs::metadata("secrets.toml")
                .expect("Should be able to get file metadata")
                .permissions();
            #[cfg(unix)]
            {
                assert_ne!(
                    0o666,
                    original_permissions.mode(),
                    "Initial file mode should not be 666"
                );
                original_permissions.set_mode(0o100_666);
                std::fs::set_permissions("secrets.toml", original_permissions.clone())
                    .expect("Should be able to set file permissions");
            }
            jail.set_env("QCS_SECRETS_FILE_PATH", "secrets.toml");
            jail.set_env("QCS_PROFILE_NAME", "test");

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Create array of token updates with different timestamps
                let token_updates = [
                    ("new_access_token", max_rfc3339()),
                    ("stale_access_token", OffsetDateTime::now_utc()),
                ];

                for (access_token, updated_at) in token_updates {
                    Secrets::write_tokens(
                        "secrets.toml",
                        "test",
                        None,
                        &SecretAccessToken::from(access_token),
                        updated_at,
                    )
                    .await
                    .expect("Should be able to write access token");
                }

                // Verify the final state
                let mut secrets = Secrets::load_from_path(&"secrets.toml".into()).unwrap();
                let payload = secrets
                    .credentials
                    .remove("test")
                    .unwrap()
                    .token_payload
                    .unwrap();

                assert_eq!(
                    payload.access_token.unwrap(),
                    SecretAccessToken::from("new_access_token")
                );
                assert_eq!(payload.updated_at.unwrap(), max_rfc3339());
                let new_permissions = std::fs::metadata("secrets.toml")
                    .expect("Should be able to get file metadata")
                    .permissions();
                assert_eq!(
                    original_permissions, new_permissions,
                    "Final file permissions should not be changed"
                );
            });

            Ok(())
        });
    }
}
