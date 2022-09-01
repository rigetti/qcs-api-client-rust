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

use crate::models::Capture;
use crate::models::DebugMessage;
use crate::models::FlatPulse;
use crate::models::Pulse;
use crate::models::SetFrequency;
use crate::models::SetPhase;
use crate::models::SetScale;
use crate::models::ShiftFrequency;
use crate::models::ShiftPhase;
use crate::models::SwapPhases;

/// Autogenerated `oneOf` implementation of `Feedback_Instructions_inner`.
///
/// # Default impl
///
/// In order to continue implementing [`Default`] on normal schema models, `oneOf` schemas must also implement
/// [`Default`]. In keeping with the upstream templates, this override defaults to the first enum variant:
/// `FeedbackInstructionsInner::Capture`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum FeedbackInstructionsInner {
    Capture(Capture),
    DebugMessage(DebugMessage),
    FlatPulse(FlatPulse),
    Pulse(Pulse),
    SetFrequency(SetFrequency),
    SetPhase(SetPhase),
    SetScale(SetScale),
    ShiftFrequency(ShiftFrequency),
    ShiftPhase(ShiftPhase),
    SwapPhases(SwapPhases),
}

impl From<Capture> for FeedbackInstructionsInner {
    fn from(variant: Capture) -> Self {
        Self::Capture(variant)
    }
}
impl From<DebugMessage> for FeedbackInstructionsInner {
    fn from(variant: DebugMessage) -> Self {
        Self::DebugMessage(variant)
    }
}
impl From<FlatPulse> for FeedbackInstructionsInner {
    fn from(variant: FlatPulse) -> Self {
        Self::FlatPulse(variant)
    }
}
impl From<Pulse> for FeedbackInstructionsInner {
    fn from(variant: Pulse) -> Self {
        Self::Pulse(variant)
    }
}
impl From<SetFrequency> for FeedbackInstructionsInner {
    fn from(variant: SetFrequency) -> Self {
        Self::SetFrequency(variant)
    }
}
impl From<SetPhase> for FeedbackInstructionsInner {
    fn from(variant: SetPhase) -> Self {
        Self::SetPhase(variant)
    }
}
impl From<SetScale> for FeedbackInstructionsInner {
    fn from(variant: SetScale) -> Self {
        Self::SetScale(variant)
    }
}
impl From<ShiftFrequency> for FeedbackInstructionsInner {
    fn from(variant: ShiftFrequency) -> Self {
        Self::ShiftFrequency(variant)
    }
}
impl From<ShiftPhase> for FeedbackInstructionsInner {
    fn from(variant: ShiftPhase) -> Self {
        Self::ShiftPhase(variant)
    }
}
impl From<SwapPhases> for FeedbackInstructionsInner {
    fn from(variant: SwapPhases) -> Self {
        Self::SwapPhases(variant)
    }
}

impl Default for FeedbackInstructionsInner {
    fn default() -> Self {
        Self::Capture(Capture::default())
    }
}

impl FeedbackInstructionsInner {
    pub fn is_capture(&self) -> bool {
        matches!(self, Self::Capture(_))
    }

    pub fn as_capture(&self) -> Option<&Capture> {
        if let Self::Capture(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_capture(self) -> Result<Capture, Self> {
        if let Self::Capture(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_debugmessage(&self) -> bool {
        matches!(self, Self::DebugMessage(_))
    }

    pub fn as_debugmessage(&self) -> Option<&DebugMessage> {
        if let Self::DebugMessage(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_debugmessage(self) -> Result<DebugMessage, Self> {
        if let Self::DebugMessage(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_flatpulse(&self) -> bool {
        matches!(self, Self::FlatPulse(_))
    }

    pub fn as_flatpulse(&self) -> Option<&FlatPulse> {
        if let Self::FlatPulse(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_flatpulse(self) -> Result<FlatPulse, Self> {
        if let Self::FlatPulse(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_pulse(&self) -> bool {
        matches!(self, Self::Pulse(_))
    }

    pub fn as_pulse(&self) -> Option<&Pulse> {
        if let Self::Pulse(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_pulse(self) -> Result<Pulse, Self> {
        if let Self::Pulse(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_setfrequency(&self) -> bool {
        matches!(self, Self::SetFrequency(_))
    }

    pub fn as_setfrequency(&self) -> Option<&SetFrequency> {
        if let Self::SetFrequency(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_setfrequency(self) -> Result<SetFrequency, Self> {
        if let Self::SetFrequency(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_setphase(&self) -> bool {
        matches!(self, Self::SetPhase(_))
    }

    pub fn as_setphase(&self) -> Option<&SetPhase> {
        if let Self::SetPhase(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_setphase(self) -> Result<SetPhase, Self> {
        if let Self::SetPhase(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_setscale(&self) -> bool {
        matches!(self, Self::SetScale(_))
    }

    pub fn as_setscale(&self) -> Option<&SetScale> {
        if let Self::SetScale(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_setscale(self) -> Result<SetScale, Self> {
        if let Self::SetScale(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_shiftfrequency(&self) -> bool {
        matches!(self, Self::ShiftFrequency(_))
    }

    pub fn as_shiftfrequency(&self) -> Option<&ShiftFrequency> {
        if let Self::ShiftFrequency(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_shiftfrequency(self) -> Result<ShiftFrequency, Self> {
        if let Self::ShiftFrequency(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_shiftphase(&self) -> bool {
        matches!(self, Self::ShiftPhase(_))
    }

    pub fn as_shiftphase(&self) -> Option<&ShiftPhase> {
        if let Self::ShiftPhase(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_shiftphase(self) -> Result<ShiftPhase, Self> {
        if let Self::ShiftPhase(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
    pub fn is_swapphases(&self) -> bool {
        matches!(self, Self::SwapPhases(_))
    }

    pub fn as_swapphases(&self) -> Option<&SwapPhases> {
        if let Self::SwapPhases(inner) = self {
            Some(inner)
        } else {
            None
        }
    }

    pub fn into_swapphases(self) -> Result<SwapPhases, Self> {
        if let Self::SwapPhases(inner) = self {
            Ok(inner)
        } else {
            Err(self)
        }
    }
}
