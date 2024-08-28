#![allow(unused_qualifications)]
use pyo3::{
    exceptions::{PyFileNotFoundError, PyOSError, PyValueError},
    prelude::*,
};
use rigetti_pyo3::{create_init_submodule, py_function_sync_async};

use crate::{
    configuration::{
        API_URL_VAR, DEFAULT_API_URL, DEFAULT_GRPC_API_URL, DEFAULT_PROFILE_NAME,
        DEFAULT_QUILC_URL, DEFAULT_QVM_URL, DEFAULT_SECRETS_PATH, DEFAULT_SETTINGS_PATH,
        GRPC_API_URL_VAR, PROFILE_NAME_VAR, QUILC_URL_VAR, QVM_URL_VAR, SECRETS_PATH_VAR,
        SETTINGS_PATH_VAR,
    },
    impl_eq, impl_repr,
};

use super::{
    error::TokenError, settings::AuthServer, tokens::ClientCredentials, ClientConfiguration,
    ClientConfigurationBuilder, LoadError, OAuthGrant, OAuthSession, RefreshToken, TokenDispatcher,
};

create_init_submodule! {
    classes: [
        ClientConfiguration,
        PyClientConfigurationBuilder,
        AuthServer,
        OAuthSession,
        RefreshToken,
        ClientCredentials
    ],
    consts: [
        DEFAULT_API_URL,
        DEFAULT_GRPC_API_URL,
        DEFAULT_QUILC_URL,
        DEFAULT_QVM_URL,
        DEFAULT_PROFILE_NAME,
        PROFILE_NAME_VAR,
        QUILC_URL_VAR,
        QVM_URL_VAR,
        API_URL_VAR,
        GRPC_API_URL_VAR,
        SETTINGS_PATH_VAR,
        DEFAULT_SETTINGS_PATH,
        SECRETS_PATH_VAR,
        DEFAULT_SECRETS_PATH
    ],
}

impl_eq!(RefreshToken);
impl_repr!(RefreshToken);
#[pymethods]
impl RefreshToken {
    #[new]
    const fn __new__(refresh_token: String) -> Self {
        Self::new(refresh_token)
    }

    #[getter]
    #[pyo3(name = "refresh_token")]
    fn py_refresh_token(&self) -> &str {
        &self.refresh_token
    }

    #[setter]
    #[pyo3(name = "refresh_token")]
    fn py_set_refresh_token(&mut self, refresh_token: String) {
        self.refresh_token = refresh_token;
    }
}

impl_eq!(ClientCredentials);
impl_repr!(ClientCredentials);
#[pymethods]
impl ClientCredentials {
    #[new]
    const fn __new__(client_id: String, client_secret: String) -> Self {
        Self::new(client_id, client_secret)
    }

    #[getter]
    #[pyo3(name = "client_id")]
    fn py_client_id(&self) -> &str {
        self.client_id()
    }

    #[getter]
    #[pyo3(name = "client_secret")]
    fn py_client_secret(&self) -> &str {
        self.client_secret()
    }
}

impl_eq!(OAuthSession);
impl_repr!(OAuthSession);
#[pymethods]
impl OAuthSession {
    #[new]
    const fn __new__(
        payload: OAuthGrant,
        auth_server: AuthServer,
        access_token: Option<String>,
    ) -> Self {
        Self::new(payload, auth_server, access_token)
    }

    #[getter]
    #[pyo3(name = "access_token")]
    fn py_access_token(&self) -> Result<&str, TokenError> {
        self.access_token()
    }

    #[getter]
    #[pyo3(name = "payload")]
    fn py_payload(&self, py: Python<'_>) -> PyObject {
        match self.payload() {
            OAuthGrant::ClientCredentials(ref client_credentials) => {
                client_credentials.clone().into_py(py)
            }
            OAuthGrant::RefreshToken(ref refresh_token) => refresh_token.clone().into_py(py),
        }
    }

    #[getter]
    #[pyo3(name = "auth_server")]
    fn py_auth_server(&self) -> AuthServer {
        self.auth_server().clone()
    }

    #[pyo3(name = "validate")]
    fn py_validate(&self) -> Result<String, TokenError> {
        self.validate()
    }
}

py_function_sync_async! {
    #[pyfunction]
    async fn get_oauth_session(tokens: Option<TokenDispatcher>) -> PyResult<OAuthSession> {
        Ok(tokens.ok_or(TokenError::NoRefreshToken)?.tokens().await)
    }
}

py_function_sync_async! {
    #[pyfunction]
    async fn get_bearer_access_token(configuration: ClientConfiguration) -> PyResult<String> {
        configuration.get_bearer_access_token().await.map_err(PyErr::from)
    }
}

impl_repr!(ClientConfiguration);
#[pymethods]
impl ClientConfiguration {
    #[staticmethod]
    #[pyo3(name = "load_default")]
    fn py_load_default() -> Result<Self, LoadError> {
        Self::load_default()
    }

    #[staticmethod]
    #[pyo3(name = "builder")]
    fn py_builder() -> PyClientConfigurationBuilder {
        PyClientConfigurationBuilder::default()
    }

    #[staticmethod]
    #[pyo3(name = "load_profile")]
    fn py_load_profile(profile_name: String) -> Result<Self, LoadError> {
        Self::load_profile(profile_name)
    }

    #[getter]
    fn get_api_url(&self) -> &str {
        &self.api_url
    }

    #[getter]
    fn get_grpc_api_url(&self) -> &str {
        &self.grpc_api_url
    }

    #[getter]
    fn get_quilc_url(&self) -> &str {
        &self.quilc_url
    }

    #[getter]
    fn get_qvm_url(&self) -> &str {
        &self.qvm_url
    }

    #[pyo3(name = "get_bearer_access_token")]
    fn py_get_bearer_access_token(&self, py: Python<'_>) -> PyResult<String> {
        py_get_bearer_access_token(py, self.clone())
    }

    #[pyo3(name = "get_bearer_access_token_async")]
    fn py_get_bearer_access_token_async<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        py_get_bearer_access_token_async(py, self.clone())
    }

    /// Get the configured tokens.
    ///
    /// # Errors
    ///
    /// - Raises a TokenError if there is a problem fetching the tokens
    pub fn get_oauth_session(&self, py: Python<'_>) -> PyResult<OAuthSession> {
        py_get_oauth_session(py, self.oauth_session.clone())
    }

    #[allow(clippy::needless_pass_by_value)] // self_ must be passed by value
    fn get_oauth_session_async<'a>(
        self_: PyRefMut<'a, Self>,
        py: Python<'a>,
    ) -> PyResult<&'a PyAny> {
        py_get_oauth_session_async(py, self_.oauth_session.clone())
    }
}

#[pyclass]
#[pyo3(name = "ClientConfigurationBuilder")]
#[derive(Clone, Default)]
struct PyClientConfigurationBuilder(ClientConfigurationBuilder);

#[pymethods]
impl PyClientConfigurationBuilder {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn build(&self) -> Result<ClientConfiguration, LoadError> {
        Ok(self.0.build()?)
    }

    #[setter]
    fn api_url(&mut self, api_url: String) {
        self.0.api_url(api_url);
    }

    #[setter]
    fn grpc_api_url(&mut self, grpc_api_url: String) {
        self.0.grpc_api_url(grpc_api_url);
    }

    #[setter]
    fn quilc_url(&mut self, quilc_url: String) {
        self.0.quilc_url(quilc_url);
    }

    #[setter]
    fn qvm_url(&mut self, qvm_url: String) {
        self.0.qvm_url(qvm_url);
    }

    #[setter]
    fn oauth_session(&mut self, oauth_session: Option<OAuthSession>) {
        self.0.oauth_session(oauth_session);
    }
}

impl_repr!(AuthServer);
impl_eq!(AuthServer);
#[pymethods]
impl AuthServer {
    #[new]
    const fn py_new(client_id: String, issuer: String) -> Self {
        Self::new(client_id, issuer)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    fn py_default() -> Self {
        Self::default()
    }

    /// Get the configured Okta client id.
    #[getter]
    #[must_use]
    pub fn get_client_id(&self) -> &str {
        self.client_id()
    }

    /// Set an Okta client id.
    #[setter(client_id)]
    pub fn py_set_client_id(&mut self, id: String) {
        self.set_client_id(id);
    }

    /// Get the Okta issuer URL.
    #[getter]
    #[must_use]
    pub fn get_issuer(&self) -> &str {
        self.issuer()
    }

    /// Set an Okta issuer URL.
    #[setter(issuer)]
    pub fn py_set_issuer(&mut self, issuer: String) {
        self.set_issuer(issuer);
    }
}

impl From<LoadError> for PyErr {
    fn from(value: LoadError) -> Self {
        let message = value.to_string();
        match value {
            LoadError::Load(_)
            | LoadError::Build(_)
            | LoadError::ProfileNotFound(_)
            | LoadError::AuthServerNotFound(_) => PyValueError::new_err(message),
            LoadError::EnvVar { .. } => PyOSError::new_err(message),
            LoadError::Path { .. } => PyFileNotFoundError::new_err(message),
            #[cfg(feature = "tracing-config")]
            LoadError::TracingFilterParseError(_) => PyValueError::new_err(message),
        }
    }
}

impl From<TokenError> for PyErr {
    fn from(value: TokenError) -> Self {
        let message = value.to_string();
        match value {
            TokenError::NoRefreshToken | TokenError::NoCredentials | TokenError::NoAccessToken | TokenError::NoAuthServer | TokenError::InvalidAccessToken(_) | TokenError::Fetch(_) => {
                PyValueError::new_err(message)
            }
            #[cfg(feature = "tonic")]
            TokenError::Transport(e) => pyo3::exceptions::PyRuntimeError::new_err(
                "Unexpected error type received, this is a bug within the qcs-api-client-common package. Please create an issue: {e}",
            ),
        }
    }
}
