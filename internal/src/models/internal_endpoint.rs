/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation. 
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// InternalEndpoint : An extension of the public Endpoint class which includes information for internal use.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InternalEndpoint {
    /// Network address at which the endpoint is locally reachable
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "addresses")]
    pub addresses: Box<crate::models::EndpointAddresses>,
    #[serde(rename = "configuration")]
    pub configuration: Box<crate::models::EndpointConfiguration>,
    /// Datacenter within which the endpoint is deployed
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    #[serde(rename = "deployment")]
    pub deployment: Box<crate::models::EndpointDeployment>,
    /// Whether the endpoint is operating as intended
    #[serde(rename = "healthy")]
    pub healthy: bool,
    /// Unique, opaque identifier for the endpoint
    #[serde(rename = "id")]
    pub id: String,
    /// Whether the endpoint serves simulated or substituted data for testing purposes
    #[serde(rename = "mock")]
    pub mock: bool,
    /// Public identifiers for quantum processors served by this endpoint.
    #[serde(rename = "quantumProcessorIds", skip_serializing_if = "Option::is_none")]
    pub quantum_processor_ids: Option<Vec<String>>,
}

impl InternalEndpoint {
    /// An extension of the public Endpoint class which includes information for internal use.
    pub fn new(address: String, addresses: crate::models::EndpointAddresses, configuration: crate::models::EndpointConfiguration, deployment: crate::models::EndpointDeployment, healthy: bool, id: String, mock: bool) -> InternalEndpoint {
        InternalEndpoint {
            address,
            addresses: Box::new(addresses),
            configuration: Box::new(configuration),
            datacenter: None,
            deployment: Box::new(deployment),
            healthy,
            id,
            mock,
            quantum_processor_ids: None,
        }
    }
}


