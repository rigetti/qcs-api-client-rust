/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://github.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://github.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// TemplateName : An enumeration.
use serde::{Deserialize, Serialize};

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateName {
    #[serde(rename = "production.controller-service.v1")]
    ProductionControllerServiceV1,
    #[serde(rename = "quantum-engineering.controller-service.v1")]
    QuantumEngineeringControllerServiceV1,
    #[serde(rename = "production.lodgepole.v1")]
    ProductionLodgepoleV1,
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "controllerMigration")]
    ControllerMigration,
    #[serde(rename = "test.magneto-backed-controller.v1")]
    TestMagnetoBackedControllerV1,
    #[serde(rename = "test.magneto-backed-controller.v2")]
    TestMagnetoBackedControllerV2,
    #[serde(rename = "test.virtual-controller.v1")]
    TestVirtualControllerV1,
}

impl std::fmt::Display for TemplateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductionControllerServiceV1 => write!(f, "production.controller-service.v1"),
            Self::QuantumEngineeringControllerServiceV1 => {
                write!(f, "quantum-engineering.controller-service.v1")
            }
            Self::ProductionLodgepoleV1 => write!(f, "production.lodgepole.v1"),
            Self::Production => write!(f, "production"),
            Self::Integration => write!(f, "integration"),
            Self::ControllerMigration => write!(f, "controllerMigration"),
            Self::TestMagnetoBackedControllerV1 => write!(f, "test.magneto-backed-controller.v1"),
            Self::TestMagnetoBackedControllerV2 => write!(f, "test.magneto-backed-controller.v2"),
            Self::TestVirtualControllerV1 => write!(f, "test.virtual-controller.v1"),
        }
    }
}

impl Default for TemplateName {
    fn default() -> TemplateName {
        Self::ProductionControllerServiceV1
    }
}
