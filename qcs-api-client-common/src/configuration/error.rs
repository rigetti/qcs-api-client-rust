use std::{error::Error, path::PathBuf};

use crate::configuration::{oidc::DISCOVERY_REQUIRED_SCOPE, tokens::PkceFlowError};

use super::secrets::SECRETS_READ_ONLY_VAR;
use super::ClientConfigurationBuilderError;

/// Errors that can occur when loading a configuration.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LoadError {
    /// Failed to load config from a file.
    #[error("Failed to load settings: {0}")]
    Load(Box<dyn Error + Send + Sync + 'static>),
    /// Failed to access or parse an environment variable.
    #[error("Failed to load value from the environment variable {variable_name}: {message}")]
    EnvVar {
        /// The name of the environment variable.
        variable_name: String,
        /// The error message.
        message: String,
    },
    /// Failed to load a file from a path.
    #[error("Failed to load file from path {path:?}: {message}")]
    Path {
        /// The path that could not be loaded.
        path: PathBuf,
        /// The error message.
        message: String,
    },
    /// The file could not be read or written to.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Failed to use the builder to build a configuration.
    #[error("Failed to build the ClientConfiguration: {0}")]
    Build(#[from] ClientConfigurationBuilderError),
    /// Provided profile not found.
    #[error("Expected profile {0} in settings.profiles but it does not exist")]
    ProfileNotFound(String),
    /// Provided authorization server not found.
    #[error("Expected auth server {0} in settings.auth_servers but it does not exist")]
    AuthServerNotFound(String),
    /// Failed to complete a PKCE login flow.
    #[error("Failed to complete PKCE login: {0}")]
    PkceFlow(#[from] PkceFlowError),
    #[cfg(feature = "tracing-config")]
    /// Failed to parse tracing filter. These should be a comma separated list of URL patterns. See
    /// <https://wicg.github.io/urlpattern> for reference.
    #[error("Could not parse tracing filter: {0}")]
    TracingFilterParseError(#[from] crate::tracing_configuration::TracingFilterError),
}

impl<E: Error + 'static> From<shellexpand::LookupError<E>> for LoadError {
    fn from(value: shellexpand::LookupError<E>) -> Self {
        Self::EnvVar {
            variable_name: value.var_name,
            message: value.cause.to_string(),
        }
    }
}

impl From<figment::Error> for LoadError {
    fn from(value: figment::Error) -> Self {
        Self::Load(Box::new(value))
    }
}

/// Errors that can occur when managing authorization tokens.
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    /// No QCS API refresh token to use.
    #[error("No refresh token is configured within the selected QCS credential.")]
    NoRefreshToken,
    /// No access token to use.
    #[error("No access token has been requested.")]
    NoAccessToken,
    /// No access token to use.
    #[error("Requested an access token for a configuration without credentials.")]
    NoCredentials,
    /// Access token is invalid.
    #[error("The access token is invalid: {0}")]
    InvalidAccessToken(jsonwebtoken::errors::Error),
    /// No QCS API refresh token to use.
    #[error("No auth server is configured within the selected QCS credential.")]
    NoAuthServer,
    /// Failure fetching a refreshed access token from the QCS API.
    #[error("Error fetching new token from the QCS API: {0}")]
    Fetch(#[from] reqwest::Error),
    /// Catch all for errors returned from an [`super::ExternallyManaged`] refresh function.
    #[error("Failed to request an externally managed access token: {0}")]
    ExternallyManaged(String),
    /// Failure writing the new access token to the secrets file.
    #[error("Failed to write the new access token to the secrets file. Setting `{SECRETS_READ_ONLY_VAR}=true` in the environment will skip persistence of newly acquired tokens. Error details: {0}")]
    Write(#[from] WriteError),
    /// Failure fetching the OIDC discovery document.
    #[error("Failed to fetch the OIDC discovery document: {0}")]
    Discovery(#[from] DiscoveryError),
}

/// Errors that can occur when attempting to fetch and process an OIDC discovery document.
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("invalid issuer URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("error fetching discovery document: {0}")]
    Fetch(#[from] reqwest::Error),
    #[error("failed to parse discovery document: {0}")]
    Json(#[from] serde_json::Error),
    #[error("issuer URL ({issuer}) is invalid: {reason}")]
    InvalidIssuer { issuer: String, reason: String },
    #[error("discovery document is invalid: {reason}")]
    InvalidDocument { reason: String },
    #[error("discovery document issuer ({document}) does not match the queried issuer ({query})")]
    IssuerMismatch { document: String, query: String },
    #[error("discovery document `supported_scopes` does not include the required minimum scope \"{expected}\", received: {0:?}", expected = DISCOVERY_REQUIRED_SCOPE)]
    InvalidScopes(Vec<String>),
}

/// Errors that can occur when trying to write or update a configuration file.
#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    /// There was an IO error while updating the secrets file.
    #[error(transparent)]
    IoWithPath(#[from] IoErrorWithPath),
    /// The file's contents are not valid TOML
    #[error("File could not be read as TOML: {0}")]
    InvalidToml(#[from] toml_edit::TomlError),
    /// TOML table could not be found.
    #[error("The table `{0}` does not exist.")]
    MissingTable(String),
    /// There was an error with time formatting
    #[error("Error formatting time: {0}.")]
    TimeFormat(#[from] time::error::Format),
    /// There was an error writing or persisting the temporary secrets file during access token refresh.
    #[error("Error writing or persisting temporary secrets file during access token refresh: {0}")]
    TempFile(#[from] async_tempfile::Error),
}

/// A fallible IO operation that can result in a [`IoErrorWithPath`]
#[derive(Debug)]
pub enum IoOperation {
    Open,
    Read,
    Write,
    Rename { dest: PathBuf },
    GetMetadata,
    SetPermissions,
    Flush,
}

/// An error wrapping [`std::io::Error`] that includes the path and operation as additional context.
#[derive(Debug, thiserror::Error)]
#[error("Io error while error performing {operation:?} on {path}: {error}")]
pub struct IoErrorWithPath {
    #[source]
    pub error: std::io::Error,
    pub path: PathBuf,
    pub operation: IoOperation,
}
