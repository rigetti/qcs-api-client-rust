use std::{error::Error, path::PathBuf};

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
    /// Failed to use the builder to build a configuration.
    #[error("Failed to build the ClientConfiguration: {0}")]
    Build(#[from] ClientConfigurationBuilderError),
    /// Provided profile not found.
    #[error("Expected profile {0} in settings.profiles but it does not exist")]
    ProfileNotFound(String),
    /// Provided authorization server not found.
    #[error("Expected auth server {0} in settings.auth_servers but it does not exist")]
    AuthServerNotFound(String),

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
}
