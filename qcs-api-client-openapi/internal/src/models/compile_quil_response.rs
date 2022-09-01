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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CompileQuilResponse {
    #[serde(rename = "nativeQuilMetadata")]
    pub native_quil_metadata: Box<crate::models::NativeQuilMetadata>,
    /// The compiled program
    #[serde(rename = "program")]
    pub program: String,
    /// The readout basis for the results. This will be a tensor with the final axis length being equal to two; the first value is the real part and the second value is the imaginary part. The client is responsible for coalescing these values to a single complex number as required.
    #[serde(rename = "readoutBasis")]
    pub readout_basis: Vec<serde_json::Value>,
    /// The twirling parameters for the program
    #[serde(rename = "twirlingParameters")]
    pub twirling_parameters: Vec<::std::collections::HashMap<String, Vec<f32>>>,
}

impl CompileQuilResponse {
    pub fn new(
        native_quil_metadata: crate::models::NativeQuilMetadata,
        program: String,
        readout_basis: Vec<serde_json::Value>,
        twirling_parameters: Vec<::std::collections::HashMap<String, Vec<f32>>>,
    ) -> CompileQuilResponse {
        CompileQuilResponse {
            native_quil_metadata: Box::new(native_quil_metadata),
            program,
            readout_basis,
            twirling_parameters,
        }
    }
}
