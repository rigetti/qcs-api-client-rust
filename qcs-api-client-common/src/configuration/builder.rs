use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::ClientConfiguration;

use super::{
    settings::AuthServer, Tokens, DEFAULT_API_URL, DEFAULT_GRPC_API_URL, DEFAULT_QUILC_URL,
    DEFAULT_QVM_URL, GRPC_API_URL_VAR, QUILC_URL_VAR, QVM_URL_VAR,
};

#[cfg(feature = "tracing-config")]
use crate::tracing_configuration::TracingConfiguration;

/// Errors that may occur when building a [`ClientConfiguration`].
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BuildError {}

/// Builder for [`ClientConfiguration`] to set/override items.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct ClientConfigurationBuilder {
    tokens: Option<Arc<Mutex<Tokens>>>,
    api_url: Option<String>,
    auth_server: Option<AuthServer>,
    grpc_api_url: Option<String>,
    quilc_url: Option<String>,
    qvm_url: Option<String>,
    #[cfg(feature = "tracing-config")]
    tracing_configuration: Option<TracingConfiguration>,
}

impl From<ClientConfiguration> for ClientConfigurationBuilder {
    fn from(config: ClientConfiguration) -> Self {
        Self {
            tokens: Some(config.tokens),
            api_url: Some(config.api_url),
            auth_server: Some(config.auth_server),
            grpc_api_url: Some(config.grpc_api_url),
            quilc_url: Some(config.quilc_url),
            qvm_url: Some(config.qvm_url),
            #[cfg(feature = "tracing-config")]
            tracing_configuration: config.tracing_configuration,
        }
    }
}

impl ClientConfigurationBuilder {
    #![allow(clippy::missing_const_for_fn)]

    /// Set the [`Tokens`] used for authentication.
    #[must_use]
    pub fn set_tokens(self, tokens: Tokens) -> Self {
        self.set_tokens_arc(Arc::new(Mutex::new(tokens)))
    }

    /// Reuse the given [`Tokens`] for authentication.
    #[must_use]
    pub fn set_tokens_arc(mut self, tokens: Arc<Mutex<Tokens>>) -> Self {
        self.tokens = Some(tokens);
        self
    }

    /// Set the OpenAPI URL to use.
    #[must_use]
    pub fn set_api_url(mut self, url: String) -> Self {
        self.api_url = Some(url);
        self
    }

    /// Set the authentication server URL to use.
    #[must_use]
    pub fn set_auth_server(mut self, auth_server: AuthServer) -> Self {
        self.auth_server = Some(auth_server);
        self
    }

    /// Set the gRPC API URL to use.
    #[must_use]
    pub fn set_grpc_api_url(mut self, url: String) -> Self {
        self.grpc_api_url = Some(url);
        self
    }

    /// Set the Quilc HTTP server to use.
    #[must_use]
    pub fn set_quilc_url(mut self, url: String) -> Self {
        self.quilc_url = Some(url);
        self
    }

    /// Set the QVM HTTP server to use.
    #[must_use]
    pub fn set_qvm_url(mut self, url: String) -> Self {
        self.qvm_url = Some(url);
        self
    }

    /// Set the [`TracingConfiguration`]. If set to `None`, network API calls will not be traced.
    /// Otherwise, the given [`TracingConfiguration`] will be used to configure tracing.
    #[cfg(feature = "tracing-config")]
    #[must_use]
    pub fn set_tracing_configuration(
        mut self,
        tracing_configuration: Option<TracingConfiguration>,
    ) -> Self {
        self.tracing_configuration = tracing_configuration;
        self
    }

    /// Build the [`ClientConfiguration`].
    ///
    /// # Errors
    ///
    /// See [`BuildError`].
    pub fn build(self) -> Result<ClientConfiguration, BuildError> {
        Ok(ClientConfiguration {
            tokens: self.tokens.unwrap_or_default(),
            api_url: self
                .api_url
                .or_else(|| std::env::var(API_URL_VAR).ok())
                .unwrap_or_else(|| DEFAULT_API_URL.to_string()),
            auth_server: self.auth_server.unwrap_or_default(),
            grpc_api_url: self
                .grpc_api_url
                .or_else(|| std::env::var(GRPC_API_URL_VAR).ok())
                .unwrap_or_else(|| DEFAULT_GRPC_API_URL.to_string()),
            quilc_url: self
                .quilc_url
                .or_else(|| std::env::var(QUILC_URL_VAR).ok())
                .unwrap_or_else(|| DEFAULT_QUILC_URL.to_string()),
            qvm_url: self
                .qvm_url
                .or_else(|| std::env::var(QVM_URL_VAR).ok())
                .unwrap_or_else(|| DEFAULT_QVM_URL.to_string()),
            #[cfg(feature = "tracing-config")]
            tracing_configuration: self.tracing_configuration,
        })
    }
}

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serial_test::serial;

    fn get_tokens() -> Tokens {
        Tokens {
            bearer_access_token: String::from("custom access"),
            refresh_token: String::from("custom refresh"),
        }
    }

    fn get_tokens_arc() -> Arc<Mutex<Tokens>> {
        Arc::new(Mutex::new(Tokens {
            bearer_access_token: String::from("custom access"),
            refresh_token: String::from("custom refresh"),
        }))
    }

    fn get_auth_server() -> AuthServer {
        AuthServer::default()
            .set_client_id(String::from("fake client id"))
            .set_issuer(String::from("fake issuer"))
    }

    macro_rules! set_from_option {
        ($builder: ident, $set_fn: ident, $op: ident) => {
            if let Some(val) = $op.clone() {
                $builder = $builder.$set_fn(val);
            }
        };
    }

    macro_rules! assert_set_from_option {
        ($field: expr, $op: ident, $default: expr) => {
            if let Some(val) = $op {
                assert_eq!($field, val, "expected value to be set in builder");
            } else {
                assert_eq!($field, $default, "expected default value to be used");
            }
        };
    }

    #[rstest]
    #[serial]
    fn test_builder_sets_options_or_default(
        #[values(None, Some(get_tokens()))] tokens: Option<Tokens>,
        #[values(None, Some(get_tokens_arc()))] tokens_arc: Option<Arc<Mutex<Tokens>>>,
        #[values(None, Some(String::from("custom api url")))] api_url: Option<String>,
        #[values(None, Some(get_auth_server()))] auth_server: Option<AuthServer>,
        #[values(None, Some(String::from("custom grpc url")))] grpc_api_url: Option<String>,
        #[values(None, Some(String::from("custom quilc url")))] quilc_url: Option<String>,
        #[values(None, Some(String::from("custom qvm url")))] qvm_url: Option<String>,
    ) {
        let mut builder = ClientConfigurationBuilder::default();

        std::env::remove_var(QUILC_URL_VAR);
        std::env::remove_var(QVM_URL_VAR);
        std::env::remove_var(GRPC_API_URL_VAR);

        set_from_option!(builder, set_tokens, tokens);
        set_from_option!(builder, set_tokens_arc, tokens_arc);
        set_from_option!(builder, set_api_url, api_url);
        set_from_option!(builder, set_auth_server, auth_server);
        set_from_option!(builder, set_grpc_api_url, grpc_api_url);
        set_from_option!(builder, set_quilc_url, quilc_url);
        set_from_option!(builder, set_qvm_url, qvm_url);

        let config = builder.build().unwrap();

        {
            // Handle tokens specially
            let expected_tokens = tokens_arc.map(|arc| arc.blocking_lock().clone()).or(tokens);
            let actual_tokens = Arc::try_unwrap(config.tokens).unwrap().into_inner();
            assert_set_from_option!(actual_tokens, expected_tokens, Tokens::default());
        }

        assert_set_from_option!(config.api_url, api_url, DEFAULT_API_URL);
        assert_set_from_option!(config.auth_server, auth_server, AuthServer::default());
        assert_set_from_option!(config.grpc_api_url, grpc_api_url, DEFAULT_GRPC_API_URL);
        assert_set_from_option!(config.quilc_url, quilc_url, DEFAULT_QUILC_URL);
        assert_set_from_option!(config.qvm_url, qvm_url, DEFAULT_QVM_URL);
    }
}
