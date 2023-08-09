use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    QcsRefresh(crate::common::configuration::RefreshError),
    ResponseError(ResponseContent<T>),
    #[cfg(feature = "otel-tracing")]
    ReqwestMiddleware(anyhow::Error),
}

impl<T> Error<T> {
    pub fn status_code(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::ResponseError(err) => Some(err.status),
            _ => None,
        }
    }
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::QcsRefresh(e) => ("refresh_qcs_token", e.to_string()),
            Error::ResponseError(e) => (
                "response",
                format!("status code {}: {}", e.status, e.content),
            ),
            #[cfg(feature = "otel-tracing")]
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::QcsRefresh(e) => e,
            #[cfg(feature = "otel-tracing")]
            Error::ReqwestMiddleware(e) => e.source()?,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

#[cfg(feature = "otel-tracing")]
impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        match e {
            reqwest_middleware::Error::Reqwest(e) => Error::Reqwest(e),
            reqwest_middleware::Error::Middleware(e) => Error::ReqwestMiddleware(e),
        }
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl<T> From<crate::common::configuration::RefreshError> for Error<T> {
    fn from(e: crate::common::configuration::RefreshError) -> Self {
        Error::QcsRefresh(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod account_api;
pub mod authentication_api;
pub mod client_applications_api;
pub mod default_api;
pub mod endpoints_api;
pub mod engagements_api;
pub mod internal_api;
pub mod legacy_api;
pub mod quantum_processors_api;
pub mod reservations_api;
pub mod translation_api;

pub mod configuration;
