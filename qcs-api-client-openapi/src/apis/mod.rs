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

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
    pub retry_delay: Option<std::time::Duration>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_path_to_error::Error<serde_json::Error>),
    Io(std::io::Error),
    QcsToken(crate::common::configuration::TokenError),
    ResponseError(ResponseContent<T>),
    InvalidContentType {
        content_type: String,
        return_type: &'static str,
    },
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
            Error::QcsToken(e) => ("refresh_qcs_token", e.to_string()),
            Error::ResponseError(e) => (
                "response",
                format!("status code {}: {}", e.status, e.content),
            ),
            Error::InvalidContentType { content_type, return_type } => (
                "api",
                format!("received {content_type} content type response that cannot be converted to `{return_type}`"),
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
            Error::QcsToken(e) => e,
            #[cfg(feature = "otel-tracing")]
            Error::ReqwestMiddleware(e) => e.source()?,
            Error::InvalidContentType { .. } => return None,
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

impl<T> From<serde_path_to_error::Error<serde_json::Error>> for Error<T> {
    fn from(e: serde_path_to_error::Error<serde_json::Error>) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl<T> From<crate::common::configuration::TokenError> for Error<T> {
    fn from(e: crate::common::configuration::TokenError) -> Self {
        Error::QcsToken(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject, got: {value:#}")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

pub mod account_api;
pub mod authentication_api;
pub mod client_applications_api;
pub mod default_api;
pub mod endpoints_api;
pub mod engagements_api;
pub mod quantum_processors_api;
pub mod reservations_api;

pub mod configuration;
