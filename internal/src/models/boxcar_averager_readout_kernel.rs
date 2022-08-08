/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation. 
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// BoxcarAveragerReadoutKernel : Readout kernel that is a constant, with a normalization condition.  The normalization rescales the values such that the kernel integrates to 1, ie. makes the scale equal to 1/duration. This is useful for preserving information about the amplitude of the readout pulse that is demodulated, however this message puts the normalization on the sequencer where it suffers from discretization error and actually goes to zero amplitude at a duration of >16 microseconds.  See also https://gitlab.com/rigetti/qcs/pidgin/-/issues/328.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BoxcarAveragerReadoutKernel {
    #[serde(rename = "_type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "bias")]
    pub bias: f32,
    #[serde(rename = "detuning")]
    pub detuning: f32,
    #[serde(rename = "duration")]
    pub duration: f32,
    #[serde(rename = "frame")]
    pub frame: String,
    #[serde(rename = "phase")]
    pub phase: f32,
    #[serde(rename = "scale")]
    pub scale: f32,
}

impl BoxcarAveragerReadoutKernel {
    /// Readout kernel that is a constant, with a normalization condition.  The normalization rescales the values such that the kernel integrates to 1, ie. makes the scale equal to 1/duration. This is useful for preserving information about the amplitude of the readout pulse that is demodulated, however this message puts the normalization on the sequencer where it suffers from discretization error and actually goes to zero amplitude at a duration of >16 microseconds.  See also https://gitlab.com/rigetti/qcs/pidgin/-/issues/328.
    pub fn new(bias: f32, detuning: f32, duration: f32, frame: String, phase: f32, scale: f32) -> BoxcarAveragerReadoutKernel {
        BoxcarAveragerReadoutKernel {
            _type: None,
            bias,
            detuning,
            duration,
            frame,
            phase,
            scale,
        }
    }
}


