/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models::ArbitraryWaveform;
use crate::models::DragGaussianWaveform;
use crate::models::ErfSquareWaveform;
use crate::models::FlatWaveform;
use crate::models::GaussianWaveform;
use crate::models::HermiteGaussianWaveform;

/// Autogenerated `oneOf` implementation of `Waveforms`.
///
/// # Default impl
///
/// In order to continue implementing [`Default`] on normal schema models, `oneOf` schemas must also implement
/// [`Default`]. In keeping with the upstream templates, this override defaults to the first enum variant:
/// `Waveforms::ArbitraryWaveform`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum Waveforms {
    ArbitraryWaveform(ArbitraryWaveform),
    DragGaussianWaveform(DragGaussianWaveform),
    ErfSquareWaveform(ErfSquareWaveform),
    FlatWaveform(FlatWaveform),
    GaussianWaveform(GaussianWaveform),
    HermiteGaussianWaveform(HermiteGaussianWaveform),
}

impl From<ArbitraryWaveform> for Waveforms {
    fn from(variant: ArbitraryWaveform) -> Self {
        Self::ArbitraryWaveform(variant)
    }
}
impl From<DragGaussianWaveform> for Waveforms {
    fn from(variant: DragGaussianWaveform) -> Self {
        Self::DragGaussianWaveform(variant)
    }
}
impl From<ErfSquareWaveform> for Waveforms {
    fn from(variant: ErfSquareWaveform) -> Self {
        Self::ErfSquareWaveform(variant)
    }
}
impl From<FlatWaveform> for Waveforms {
    fn from(variant: FlatWaveform) -> Self {
        Self::FlatWaveform(variant)
    }
}
impl From<GaussianWaveform> for Waveforms {
    fn from(variant: GaussianWaveform) -> Self {
        Self::GaussianWaveform(variant)
    }
}
impl From<HermiteGaussianWaveform> for Waveforms {
    fn from(variant: HermiteGaussianWaveform) -> Self {
        Self::HermiteGaussianWaveform(variant)
    }
}

impl Default for Waveforms {
    fn default() -> Self {
        Self::ArbitraryWaveform(ArbitraryWaveform::default())
    }
}

impl Waveforms {
    pub fn is_arbitrarywaveform(&self) -> bool {
        matches!(self, Self::ArbitraryWaveform(_))
    }

    pub fn as_arbitrarywaveform(&self) -> Option<&ArbitraryWaveform> {
        if let Self::ArbitraryWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_arbitrarywaveform(self) -> Result<ArbitraryWaveform, Self> {
        if let Self::ArbitraryWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_draggaussianwaveform(&self) -> bool {
        matches!(self, Self::DragGaussianWaveform(_))
    }

    pub fn as_draggaussianwaveform(&self) -> Option<&DragGaussianWaveform> {
        if let Self::DragGaussianWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_draggaussianwaveform(self) -> Result<DragGaussianWaveform, Self> {
        if let Self::DragGaussianWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_erfsquarewaveform(&self) -> bool {
        matches!(self, Self::ErfSquareWaveform(_))
    }

    pub fn as_erfsquarewaveform(&self) -> Option<&ErfSquareWaveform> {
        if let Self::ErfSquareWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_erfsquarewaveform(self) -> Result<ErfSquareWaveform, Self> {
        if let Self::ErfSquareWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_flatwaveform(&self) -> bool {
        matches!(self, Self::FlatWaveform(_))
    }

    pub fn as_flatwaveform(&self) -> Option<&FlatWaveform> {
        if let Self::FlatWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_flatwaveform(self) -> Result<FlatWaveform, Self> {
        if let Self::FlatWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_gaussianwaveform(&self) -> bool {
        matches!(self, Self::GaussianWaveform(_))
    }

    pub fn as_gaussianwaveform(&self) -> Option<&GaussianWaveform> {
        if let Self::GaussianWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_gaussianwaveform(self) -> Result<GaussianWaveform, Self> {
        if let Self::GaussianWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_hermitegaussianwaveform(&self) -> bool {
        matches!(self, Self::HermiteGaussianWaveform(_))
    }

    pub fn as_hermitegaussianwaveform(&self) -> Option<&HermiteGaussianWaveform> {
        if let Self::HermiteGaussianWaveform(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_hermitegaussianwaveform(self) -> Result<HermiteGaussianWaveform, Self> {
        if let Self::HermiteGaussianWaveform(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
}
