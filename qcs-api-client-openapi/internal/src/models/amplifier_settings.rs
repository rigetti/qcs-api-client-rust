/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// AmplifierSettings : All information needed to operate an parametric amplifier.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AmplifierSettings {
    #[serde(rename = "_type")]
    pub _type: String,
    #[serde(rename = "output")]
    pub output: bool,
    #[serde(rename = "pump_freq")]
    pub pump_freq: f32,
    #[serde(rename = "pump_power")]
    pub pump_power: f32,
    #[serde(rename = "qubit_set")]
    pub qubit_set: Vec<i32>,
    #[serde(rename = "slow_flux", skip_serializing_if = "Option::is_none")]
    pub slow_flux: Option<Box<crate::models::SlowFluxSettings>>,
}

impl AmplifierSettings {
    /// All information needed to operate an parametric amplifier.
    pub fn new(
        _type: String,
        output: bool,
        pump_freq: f32,
        pump_power: f32,
        qubit_set: Vec<i32>,
    ) -> AmplifierSettings {
        AmplifierSettings {
            _type,
            output,
            pump_freq,
            pump_power,
            qubit_set,
            slow_flux: None,
        }
    }
}
