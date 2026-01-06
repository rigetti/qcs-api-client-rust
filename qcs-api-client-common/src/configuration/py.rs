#![allow(unused_qualifications)]
#![allow(non_local_definitions, reason = "necessary for pyo3::pymethods")]

use pyo3::{
    exceptions::{PyFileNotFoundError, PyOSError, PyRuntimeError, PyValueError},
    prelude::*,
    types::PyFunction,
};
use pyo3_asyncio::tokio::get_runtime;
use rigetti_pyo3::{create_init_submodule, py_function_sync_async};
use tokio_util::sync::CancellationToken;

use crate::{
    configuration::{
        secrets::{DEFAULT_SECRETS_PATH, SECRETS_PATH_VAR},
        settings::{DEFAULT_SETTINGS_PATH, SETTINGS_PATH_VAR},
        API_URL_VAR, DEFAULT_API_URL, DEFAULT_GRPC_API_URL, DEFAULT_PROFILE_NAME,
        DEFAULT_QUILC_URL, DEFAULT_QVM_URL, GRPC_API_URL_VAR, PROFILE_NAME_VAR, QUILC_URL_VAR,
        QVM_URL_VAR,
    },
    impl_eq, impl_repr,
};

use super::{
    error::TokenError,
    secrets::{SecretAccessToken, SecretRefreshToken},
    settings::AuthServer,
    tokens::{ClientCredentials, ClientSecret, ExternallyManaged, PkceFlow},
    ClientConfiguration, ClientConfigurationBuilder, LoadError, OAuthGrant, OAuthSession,
    RefreshToken, TokenDispatcher,
};

create_init_submodule! {
    classes: [
        ClientConfiguration,
        PyClientConfigurationBuilder,
        AuthServer,
        OAuthSession,
        RefreshToken,
        ClientCredentials,
        ClientSecret,
        ExternallyManaged,
        PkceFlow,
        SecretAccessToken,
        SecretRefreshToken
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
    const fn __new__(refresh_token: SecretRefreshToken) -> Self {
        Self::new(refresh_token)
    }

    #[getter]
    #[pyo3(name = "refresh_token")]
    fn py_refresh_token(&self) -> SecretRefreshToken {
        self.refresh_token.clone()
    }

    #[setter]
    #[pyo3(name = "refresh_token")]
    fn py_set_refresh_token(&mut self, refresh_token: SecretRefreshToken) {
        self.refresh_token = refresh_token;
    }
}

impl_eq!(ClientCredentials);
impl_repr!(ClientCredentials);
#[pymethods]
impl ClientCredentials {
    #[new]
    fn __new__(client_id: String, client_secret: String) -> Self {
        Self::new(client_id, ClientSecret::from(client_secret))
    }

    #[getter]
    #[pyo3(name = "client_id")]
    fn py_client_id(&self) -> &str {
        self.client_id()
    }

    #[getter]
    #[pyo3(name = "client_secret")]
    fn py_client_secret(&self) -> ClientSecret {
        self.client_secret().clone()
    }
}

impl_repr!(ExternallyManaged);
#[pymethods]
impl ExternallyManaged {
    #[new]
    fn __new__(refresh_function: Py<PyFunction>) -> Self {
        #[allow(trivial_casts)] // Compilation fails without the cast.
        // The provided refresh function will panic if there is an issue with the refresh function.
        // This raises a `PanicException` within Python.
        let refresh_closure = move |auth_server: AuthServer| {
            let refresh_function = refresh_function.clone();
            Box::pin(async move {
                Python::with_gil(|py| {
                    let result = refresh_function.call1(py, (auth_server.into_py(py),));
                    match result {
                        Ok(value) => value
                            .extract::<String>(py)
                            .map_or_else(|_| panic!("ExternallyManaged refresh function returned an unexpected type. Expected a string, got {value:?}"), Ok),
                        Err(err) => Err(Box::<dyn std::error::Error + Send + Sync>::from(err))
                    }
                })
            }) as super::tokens::RefreshResult
        };

        Self::new(refresh_closure)
    }
}

impl_eq!(PkceFlow);
// Does not implement `__repr__`, since the data contains a secret value.
#[pymethods]
impl PkceFlow {
    #[new]
    fn __new__(py: Python<'_>, auth_server: AuthServer) -> PyResult<Self> {
        py.allow_threads(move || {
            let runtime = get_runtime();
            runtime.block_on(async move {
                let cancel_token = cancel_token_with_ctrl_c();
                Self::new_login_flow(cancel_token, &auth_server).await
            })
        })
        .map_err(|err| PyRuntimeError::new_err(err.to_string()))
    }

    #[getter]
    #[pyo3(name = "access_token")]
    fn py_access_token(&self) -> SecretAccessToken {
        self.access_token.clone()
    }

    #[getter]
    #[pyo3(name = "refresh_token")]
    fn py_refresh_token(&self) -> Option<SecretRefreshToken> {
        self.refresh_token
            .as_ref()
            .map(|rt| rt.refresh_token.clone())
    }
}

impl_repr!(OAuthSession);
#[pymethods]
impl OAuthSession {
    #[new]
    const fn __new__(
        payload: OAuthGrant,
        auth_server: AuthServer,
        access_token: Option<SecretAccessToken>,
    ) -> Self {
        Self::new(payload, auth_server, access_token)
    }

    #[getter]
    #[pyo3(name = "access_token")]
    fn py_access_token(&self) -> Result<SecretAccessToken, TokenError> {
        self.access_token().cloned()
    }

    #[getter]
    #[pyo3(name = "payload")]
    fn py_payload(&self, py: Python<'_>) -> PyObject {
        match self.payload() {
            OAuthGrant::ClientCredentials(ref client_credentials) => {
                client_credentials.clone().into_py(py)
            }
            OAuthGrant::RefreshToken(ref refresh_token) => refresh_token.clone().into_py(py),
            OAuthGrant::ExternallyManaged(ref externally_managed) => {
                externally_managed.clone().into_py(py)
            }
            OAuthGrant::PkceFlow(ref pkce_tokens) => pkce_tokens.clone().into_py(py),
        }
    }

    #[getter]
    #[pyo3(name = "auth_server")]
    fn py_auth_server(&self) -> AuthServer {
        self.auth_server().clone()
    }

    #[pyo3(name = "validate")]
    fn py_validate(&self) -> Result<SecretAccessToken, TokenError> {
        self.validate()
    }

    #[pyo3(name = "request_access_token")]
    fn py_request_access_token(&self, py: Python<'_>) -> PyResult<SecretAccessToken> {
        py_request_access_token(py, self.clone())
    }

    #[pyo3(name = "request_access_token_async")]
    fn py_request_access_token_async<'a>(&'a self, py: Python<'a>) -> PyResult<&'a PyAny> {
        py_request_access_token_async(py, self.clone())
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
    async fn get_bearer_access_token(configuration: ClientConfiguration) -> PyResult<SecretAccessToken> {
        configuration.get_bearer_access_token().await.map_err(PyErr::from)
    }
}

py_function_sync_async! {
    #[pyfunction]
    async fn request_access_token(session: OAuthSession) -> PyResult<SecretAccessToken> {
        session.clone().request_access_token().await.cloned().map_err(PyErr::from)
    }
}

impl_repr!(ClientConfiguration);
#[pymethods]
impl ClientConfiguration {
    #[staticmethod]
    #[pyo3(name = "load_default")]
    fn py_load_default(_py: Python<'_>) -> Result<Self, LoadError> {
        Self::load_default()
    }

    #[staticmethod]
    #[pyo3(name = "load_default_with_login")]
    fn py_load_default_with_login(py: Python<'_>) -> PyResult<Self> {
        py.allow_threads(move || {
            let runtime = get_runtime();
            runtime.block_on(async move {
                let cancel_token = cancel_token_with_ctrl_c();
                Self::load_with_login(cancel_token, None).await
            })
        })
        .map_err(|err| PyRuntimeError::new_err(err.to_string()))
    }

    #[staticmethod]
    #[pyo3(name = "builder")]
    fn py_builder() -> PyClientConfigurationBuilder {
        PyClientConfigurationBuilder::default()
    }

    #[staticmethod]
    #[pyo3(name = "load_profile")]
    fn py_load_profile(_py: Python<'_>, profile_name: String) -> Result<Self, LoadError> {
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
    fn py_get_bearer_access_token(&self, py: Python<'_>) -> PyResult<SecretAccessToken> {
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
    /// - Raises a `TokenError` if there is a problem fetching the tokens
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
    const fn __new__(client_id: String, issuer: String, scopes: Option<Vec<String>>) -> Self {
        Self::new(client_id, issuer, scopes)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    fn py_default() -> Self {
        Self::default()
    }

    /// Get the configured OAuth OIDC client id.
    #[getter]
    #[must_use]
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    /// Set an OAuth OIDC client id.
    #[setter(client_id)]
    pub fn py_set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    /// Get the OAuth OIDC issuer URL.
    #[getter]
    #[must_use]
    pub fn get_issuer(&self) -> &str {
        &self.issuer
    }

    /// Set an OAuth OIDC issuer URL.
    #[setter(issuer)]
    pub fn py_set_issuer(&mut self, issuer: String) {
        self.issuer = issuer;
    }
}

impl From<LoadError> for PyErr {
    fn from(value: LoadError) -> Self {
        let message = value.to_string();
        match value {
            LoadError::Load(_)
            | LoadError::Build(_)
            | LoadError::ProfileNotFound(_)
            | LoadError::AuthServerNotFound(_)
            | LoadError::PkceFlow(_) => PyValueError::new_err(message),
            LoadError::EnvVar { .. } | LoadError::Io(_) => PyOSError::new_err(message),
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
            TokenError::NoRefreshToken
            | TokenError::NoCredentials
            | TokenError::NoAccessToken
            | TokenError::NoAuthServer
            | TokenError::InvalidAccessToken(_)
            | TokenError::Fetch(_)
            | TokenError::ExternallyManaged(_)
            | TokenError::Write(_)
            | TokenError::Discovery(_) => PyValueError::new_err(message),
        }
    }
}

fn cancel_token_with_ctrl_c() -> CancellationToken {
    let cancel_token = CancellationToken::new();
    let cancel_token_ctrl_c = cancel_token.clone();
    tokio::spawn(cancel_token.clone().run_until_cancelled_owned(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => cancel_token_ctrl_c.cancel(),
            Err(error) => eprintln!("Failed to register signal handler: {error}"),
        }
    }));
    cancel_token
}
