/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation. 
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// DragGaussianWaveform : Gaussian-like waveform used for transmons to correct for the transmon's anharmonicity.  DRAG === Derivative Removal by Adiabatic Gate.  Reference: Motzoi, et al, Simple Pulses for Elimination of Leakage in Weakly Nonlinear Qubits. DOI: 10.1103/PhysRevLett.103.110501



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DragGaussianWaveform {
    #[serde(rename = "_type")]
    pub _type: String,
    #[serde(rename = "alpha")]
    pub alpha: f32,
    #[serde(rename = "anh")]
    pub anh: f32,
    #[serde(rename = "detuning")]
    pub detuning: f32,
    #[serde(rename = "duration")]
    pub duration: f32,
    #[serde(rename = "frame")]
    pub frame: String,
    #[serde(rename = "fwhm")]
    pub fwhm: f32,
    #[serde(rename = "phase")]
    pub phase: f32,
    #[serde(rename = "scale")]
    pub scale: f32,
    #[serde(rename = "t0")]
    pub t0: f32,
}

impl DragGaussianWaveform {
    /// Gaussian-like waveform used for transmons to correct for the transmon's anharmonicity.  DRAG === Derivative Removal by Adiabatic Gate.  Reference: Motzoi, et al, Simple Pulses for Elimination of Leakage in Weakly Nonlinear Qubits. DOI: 10.1103/PhysRevLett.103.110501
    pub fn new(_type: String, alpha: f32, anh: f32, detuning: f32, duration: f32, frame: String, fwhm: f32, phase: f32, scale: f32, t0: f32) -> DragGaussianWaveform {
        DragGaussianWaveform {
            _type,
            alpha,
            anh,
            detuning,
            duration,
            frame,
            fwhm,
            phase,
            scale,
            t0,
        }
    }
}


