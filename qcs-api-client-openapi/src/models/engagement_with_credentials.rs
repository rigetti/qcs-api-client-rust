/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// EngagementWithCredentials : An engagement is the authorization of a user to execute work on a Quantum Processor Endpoint.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EngagementWithCredentials {
    /// User ID or group name on behalf of which the engagement is made.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Indicates whether the grant was made on behalf of a single user or group.
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<Box<crate::models::AccountType>>,
    /// The network address of the endpoint to which this engagement grants access
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "credentials")]
    pub credentials: Box<crate::models::EngagementCredentials>,
    /// The ID of the endpoint to which this engagement grants access
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    /// Time after which the engagement is no longer valid. Given in RFC3339 format.
    #[serde(rename = "expiresAt")]
    pub expires_at: String,
    /// The minimum priority value allowed for execution
    #[serde(rename = "minimumPriority", skip_serializing_if = "Option::is_none")]
    pub minimum_priority: Option<i32>,
    /// The quantum processors for which this engagement enables access and execution
    #[serde(
        rename = "quantumProcessorIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub quantum_processor_ids: Option<Vec<String>>,
    /// Tags recorded on QPU requests and recorded on usage records.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl EngagementWithCredentials {
    /// An engagement is the authorization of a user to execute work on a Quantum Processor Endpoint.
    pub fn new(
        address: String,
        credentials: crate::models::EngagementCredentials,
        endpoint_id: String,
        expires_at: String,
        user_id: String,
    ) -> EngagementWithCredentials {
        EngagementWithCredentials {
            account_id: None,
            account_type: None,
            address,
            credentials: Box::new(credentials),
            endpoint_id,
            expires_at,
            minimum_priority: None,
            quantum_processor_ids: None,
            tags: None,
            user_id,
        }
    }
}