/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

use crate::models::ParameterAref;
use crate::models::ParameterExpression;

/// Autogenerated `oneOf` implementation of `Delta`.
///
/// # Default impl
///
/// In order to continue implementing [`Default`] on normal schema models, `oneOf` schemas must also implement
/// [`Default`]. In keeping with the upstream templates, this override defaults to the first enum variant:
/// `Delta::ParameterAref`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Delta {
    ParameterAref(ParameterAref),
    ParameterExpression(ParameterExpression),
    F32(f32),
}

impl From<ParameterAref> for Delta {
    fn from(variant: ParameterAref) -> Self {
        Self::ParameterAref(variant)
    }
}
impl From<ParameterExpression> for Delta {
    fn from(variant: ParameterExpression) -> Self {
        Self::ParameterExpression(variant)
    }
}
impl From<f32> for Delta {
    fn from(variant: f32) -> Self {
        Self::F32(variant)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Self::ParameterAref(ParameterAref::default())
    }
}

impl Delta {
    pub fn is_parameteraref(&self) -> bool {
        matches!(self, Self::ParameterAref(_))
    }

    pub fn as_parameteraref(&self) -> Option<&ParameterAref> {
        if let Self::ParameterAref(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_parameteraref(self) -> Result<ParameterAref, Self> {
        if let Self::ParameterAref(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_parameterexpression(&self) -> bool {
        matches!(self, Self::ParameterExpression(_))
    }

    pub fn as_parameterexpression(&self) -> Option<&ParameterExpression> {
        if let Self::ParameterExpression(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_parameterexpression(self) -> Result<ParameterExpression, Self> {
        if let Self::ParameterExpression(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_f32(&self) -> bool {
        matches!(self, Self::F32(_))
    }

    pub fn as_f32(&self) -> Option<&f32> {
        if let Self::F32(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_f32(self) -> Result<f32, Self> {
        if let Self::F32(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
}
