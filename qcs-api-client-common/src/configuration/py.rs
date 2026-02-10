#![allow(unused_qualifications)]
#![allow(non_local_definitions, reason = "necessary for pyo3::pymethods")]

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyFunction, PyString},
};
use rigetti_pyo3::{create_init_submodule, impl_repr, py_function_sync_async, sync::Awaitable};
use tokio_util::sync::CancellationToken;

#[cfg(feature = "stubs")]
use pyo3_stub_gen::derive::{gen_stub_pyfunction, gen_stub_pymethods};

use crate::configuration::{
    secrets::{DEFAULT_SECRETS_PATH, SECRETS_PATH_VAR},
    settings::{DEFAULT_SETTINGS_PATH, SETTINGS_PATH_VAR},
    ClientConfigurationBuilderError, API_URL_VAR, DEFAULT_API_URL, DEFAULT_GRPC_API_URL,
    DEFAULT_PROFILE_NAME, DEFAULT_QUILC_URL, DEFAULT_QVM_URL, GRPC_API_URL_VAR, PROFILE_NAME_VAR,
    QUILC_URL_VAR, QVM_URL_VAR,
};
use crate::errors;

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
        ClientConfigurationBuilder,
        AuthServer,
        OAuthSession,
        RefreshToken,
        ClientCredentials,
        ClientSecret,
        ExternallyManaged,
        PkceFlow,
        SecretAccessToken,
        SecretRefreshToken,
        TokenDispatcher
    ],

    consts: [
        API_URL_VAR,
        DEFAULT_API_URL,
        DEFAULT_GRPC_API_URL,
        DEFAULT_PROFILE_NAME,
        DEFAULT_QUILC_URL,
        DEFAULT_QVM_URL,
        DEFAULT_SECRETS_PATH,
        DEFAULT_SETTINGS_PATH,
        GRPC_API_URL_VAR,
        PROFILE_NAME_VAR,
        QUILC_URL_VAR,
        QVM_URL_VAR,
        SECRETS_PATH_VAR,
        SETTINGS_PATH_VAR
    ],

    errors: [
        errors::ClientConfigurationBuilderError,
        errors::ConfigurationError,
        errors::LoadError,
        errors::TokenError
    ],

    funcs: [
        py_get_oauth_session,
        py_get_oauth_session_async,
        py_get_bearer_access_token,
        py_get_bearer_access_token_async,
        py_request_access_token,
        py_request_access_token_async
    ],

}

#[cfg(feature = "stubs")]
#[derive(IntoPyObject)]
struct Final<T>(T);

#[cfg(feature = "stubs")]
impl<T> pyo3_stub_gen::PyStubType for Final<T> {
    fn type_output() -> pyo3_stub_gen::TypeInfo {
        pyo3_stub_gen::TypeInfo::with_module("typing.Final", "typing".into())
    }
}

/// Adds module-level `str` to the `qcs_api_client_common.configuration` stub file.
macro_rules! stub_consts {
    ( $($name:ident),* ) => {
        $(
            #[cfg(feature = "stubs")]
            ::pyo3_stub_gen::module_variable!(
                "qcs_api_client_common.configuration",
                stringify!($name),
                Final<&str>,
                Final($name)
            );
        )*
    };
}

stub_consts!(
    API_URL_VAR,
    DEFAULT_API_URL,
    DEFAULT_GRPC_API_URL,
    DEFAULT_PROFILE_NAME,
    DEFAULT_QUILC_URL,
    DEFAULT_QVM_URL,
    DEFAULT_SECRETS_PATH,
    DEFAULT_SETTINGS_PATH,
    GRPC_API_URL_VAR,
    PROFILE_NAME_VAR,
    QUILC_URL_VAR,
    QVM_URL_VAR,
    SECRETS_PATH_VAR,
    SETTINGS_PATH_VAR
);

/// Manual implementation to extract tokens from Python objects.
///
/// For Python functions that require a `SecretRefreshToken`,
/// users can provide a Python `str`, a `RefreshToken`, or a `SecretRefreshToken`.
impl FromPyObject<'_, '_> for SecretRefreshToken {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(token) = obj.cast::<PyString>() {
            Ok(Self::__new__(token.extract()?))
        } else if let Ok(token) = obj.cast::<RefreshToken>() {
            Ok(token.borrow().refresh_token.clone())
        } else if let Ok(token) = obj.cast::<Self>() {
            Ok(token.borrow().clone())
        } else {
            Err(PyValueError::new_err(
                "expected str | SecretRefreshToken | RefreshToken",
            ))
        }
    }
}

impl FromPyObject<'_, '_> for SecretAccessToken {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(token) = obj.cast::<PyString>() {
            Ok(Self::__new__(token.extract()?))
        } else if let Ok(token) = obj.cast::<Self>() {
            Ok(token.borrow().clone())
        } else {
            Err(PyValueError::new_err("expected str | SecretAccessToken"))
        }
    }
}

impl FromPyObject<'_, '_> for ClientSecret {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(token) = obj.cast::<PyString>() {
            Ok(Self::__new__(token.extract()?))
        } else if let Ok(token) = obj.cast::<Self>() {
            Ok(token.borrow().clone())
        } else {
            Err(PyValueError::new_err("expected str | ClientSecret"))
        }
    }
}

impl_repr!(RefreshToken);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl RefreshToken {
    #[new]
    const fn __new__(refresh_token: SecretRefreshToken) -> Self {
        Self::new(refresh_token)
    }
}

impl_repr!(ClientCredentials);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl ClientCredentials {
    #[new]
    fn __new__(client_id: String, client_secret: String) -> Self {
        Self::new(client_id, ClientSecret::from(client_secret))
    }
}

impl_repr!(ExternallyManaged);

#[cfg_attr(not(feature = "stubs"), optipy::strip_pyo3(only_stubs))]
#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl ExternallyManaged {
    #[new]
    fn __new__(
        #[gen_stub(
            override_type(
                type_repr="collections.abc.Callable[[AuthServer], str]",
                imports=("collections.abc")
            )
        )]
        refresh_function: Py<PyFunction>,
    ) -> Self {
        #[allow(trivial_casts)] // Compilation fails without the cast.
        // The provided refresh function will panic if there is an issue with the refresh function.
        // This raises a `PanicException` within Python.
        let refresh_closure = move |auth_server: AuthServer| {
            let refresh_function = Python::attach(|py| refresh_function.clone_ref(py));
            Box::pin(async move {
                Python::attach(|py| {
                    let result = refresh_function.call1(py, (auth_server,));
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

impl_repr!(PkceFlow);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl PkceFlow {
    #[new]
    fn __new__(py: Python<'_>, auth_server: AuthServer) -> PyResult<Self> {
        pyo3_async_runtimes::tokio::run(py, async move {
            let cancel_token = cancel_token_with_ctrl_c();
            Self::new_login_flow(cancel_token, &auth_server)
                .await
                .map_err(|err| LoadError::from(err).into())
        })
    }
}

#[cfg(feature = "stubs")]
pyo3_stub_gen::impl_stub_type!(
    OAuthGrant = RefreshToken | ClientConfiguration | ExternallyManaged | PkceFlow
);

impl_repr!(OAuthSession);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl OAuthSession {
    #[new]
    #[pyo3(signature = (payload, auth_server, access_token = None))]
    const fn __new__(
        payload: OAuthGrant,
        auth_server: AuthServer,
        access_token: Option<SecretAccessToken>,
    ) -> Self {
        Self::new(payload, auth_server, access_token)
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
    fn py_request_access_token_async<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Awaitable<'py, SecretAccessToken>> {
        py_request_access_token_async(py, self.clone())
    }
}

py_function_sync_async! {
    #[cfg_attr(feature = "stubs", gen_stub_pyfunction(module = "qcs_api_client_common.configuration"))]
    #[pyfunction]
    async fn get_oauth_session(tokens: Option<TokenDispatcher>) -> PyResult<OAuthSession> {
        Ok(tokens.ok_or(TokenError::NoRefreshToken)?.tokens().await)
    }
}

py_function_sync_async! {
    #[cfg_attr(feature = "stubs", gen_stub_pyfunction(module = "qcs_api_client_common.configuration"))]
    #[pyfunction]
    async fn get_bearer_access_token(configuration: ClientConfiguration) -> PyResult<SecretAccessToken> {
        configuration.get_bearer_access_token().await.map_err(PyErr::from)
    }
}

py_function_sync_async! {
    #[cfg_attr(feature = "stubs", gen_stub_pyfunction(module = "qcs_api_client_common.configuration"))]
    #[pyfunction]
    async fn request_access_token(session: OAuthSession) -> PyResult<SecretAccessToken> {
        session.clone().request_access_token().await.cloned().map_err(PyErr::from)
    }
}

impl_repr!(ClientConfiguration);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl ClientConfiguration {
    #[new]
    #[pyo3(signature = (
            api_url = None, grpc_api_url = None, quilc_url = None, qvm_url = None,
            oauth_session = None,
            ))]
    fn __new__(
        api_url: Option<String>,
        grpc_api_url: Option<String>,
        quilc_url: Option<String>,
        qvm_url: Option<String>,
        oauth_session: Option<OAuthSession>,
    ) -> Self {
        let mut builder = ClientConfigurationBuilder::default();

        if let Some(api_url) = api_url {
            builder.api_url(api_url);
        }

        if let Some(grpc_api_url) = grpc_api_url {
            builder.grpc_api_url(grpc_api_url);
        }

        if let Some(quilc_url) = quilc_url {
            builder.quilc_url(quilc_url);
        }

        if let Some(qvm_url) = qvm_url {
            builder.qvm_url(qvm_url);
        }

        builder.oauth_session(oauth_session);

        builder
            .build()
            .expect("our builder is valid regardless of which URLs are set")
    }

    #[staticmethod]
    #[pyo3(name = "load_default")]
    fn py_load_default(_py: Python<'_>) -> Result<Self, LoadError> {
        Self::load_default()
    }

    #[staticmethod]
    #[pyo3(name = "load_default_with_login")]
    fn py_load_default_with_login(py: Python<'_>) -> PyResult<Self> {
        pyo3_async_runtimes::tokio::run(py, async move {
            let cancel_token = cancel_token_with_ctrl_c();
            Self::load_with_login(cancel_token, None)
                .await
                .map_err(Into::into)
        })
    }

    #[staticmethod]
    #[pyo3(name = "builder")]
    fn py_builder() -> ClientConfigurationBuilder {
        ClientConfigurationBuilder::default()
    }

    #[staticmethod]
    #[pyo3(name = "load_profile")]
    fn py_load_profile(_py: Python<'_>, profile_name: String) -> Result<Self, LoadError> {
        Self::load_profile(profile_name)
    }

    #[pyo3(name = "get_bearer_access_token")]
    fn py_get_bearer_access_token(&self, py: Python<'_>) -> PyResult<SecretAccessToken> {
        py_get_bearer_access_token(py, self.clone())
    }

    #[pyo3(name = "get_bearer_access_token_async")]
    fn py_get_bearer_access_token_async<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Awaitable<'py, SecretAccessToken>> {
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
    fn get_oauth_session_async<'py>(
        self_: PyRefMut<'py, Self>,
        py: Python<'py>,
    ) -> PyResult<Awaitable<'py, OAuthSession>> {
        py_get_oauth_session_async(py, self_.oauth_session.clone())
    }
}

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl ClientConfigurationBuilder {
    #[new]
    fn __new__() -> Self {
        Self::default()
    }

    /// The [`OAuthSession`] to use to authenticate with the QCS API.
    ///
    /// When set to [`None`], the configuration will not manage an OAuth Session, and access to the
    /// QCS API will be limited to unauthenticated routes.
    #[setter]
    fn set_oauth_session(&mut self, oauth_session: Option<OAuthSession>) {
        self.oauth_session = Some(oauth_session.map(Into::into));
    }

    #[pyo3(name = "build")]
    fn py_build(&self) -> Result<ClientConfiguration, ClientConfigurationBuilderError> {
        self.build()
    }
}

impl_repr!(AuthServer);

#[cfg_attr(feature = "stubs", gen_stub_pymethods)]
#[pymethods]
impl AuthServer {
    #[new]
    #[pyo3(signature = (client_id, issuer, scopes = None))]
    const fn __new__(client_id: String, issuer: String, scopes: Option<Vec<String>>) -> Self {
        Self::new(client_id, issuer, scopes)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    fn py_default() -> Self {
        Self::default()
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
