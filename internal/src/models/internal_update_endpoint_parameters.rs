/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation. 
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InternalUpdateEndpointParameters {
    #[serde(rename = "componentParameters", skip_serializing_if = "Option::is_none")]
    pub component_parameters: Option<Box<crate::models::ComponentParameters>>,
    /// Which datacenters are available for endpoint placement. Defaults to all available datacenters for non mock endpoints and berkeley-775 for mock endpoints.
    #[serde(rename = "datacenters", skip_serializing_if = "Option::is_none")]
    pub datacenters: Option<Vec<crate::models::NomadJobDatacenters>>,
    /// The name of the policy used to enforce engagements on this endpoint
    #[serde(rename = "engagementPolicyName", skip_serializing_if = "Option::is_none")]
    pub engagement_policy_name: Option<Box<crate::models::EngagementPolicyName>>,
    #[serde(rename = "engagementPolicyOptions", skip_serializing_if = "Option::is_none")]
    pub engagement_policy_options: Option<Box<crate::models::EngagementPolicyOptions>>,
    /// Identifier for the dilution fridge system to which this endpoint has network access
    #[serde(rename = "fridgeId", skip_serializing_if = "Option::is_none")]
    pub fridge_id: Option<String>,
    /// Whether these components should have access to live instruments on the corresponding fridge
    #[serde(rename = "liveInstrumentAccess", skip_serializing_if = "Option::is_none")]
    pub live_instrument_access: Option<bool>,
    /// Public identifiers for quantum processors served by this endpoint. Note: this is only used for QCS services and authorization and does not generally affect jobs run by internally-authorized users.
    #[serde(rename = "quantumProcessorIds", skip_serializing_if = "Option::is_none")]
    pub quantum_processor_ids: Option<Vec<String>>,
    /// The name of the template used to apply default values to this endpoint
    #[serde(rename = "templateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<Box<crate::models::TemplateName>>,
}

impl InternalUpdateEndpointParameters {
    pub fn new() -> InternalUpdateEndpointParameters {
        InternalUpdateEndpointParameters {
            component_parameters: None,
            datacenters: None,
            engagement_policy_name: None,
            engagement_policy_options: None,
            fridge_id: None,
            live_instrument_access: None,
            quantum_processor_ids: None,
            template_name: None,
        }
    }
}


