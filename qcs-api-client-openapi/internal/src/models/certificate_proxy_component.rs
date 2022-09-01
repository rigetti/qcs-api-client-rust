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
pub struct CertificateProxyComponent {
    /// CPU Allocation, in MHz, required by this component. Whether it is a hard or soft limit is specified by the component itself. By default, it is a soft limit, and components are allowed to burst above when there is unused capacity.
    #[serde(rename = "cpuLimit", skip_serializing_if = "Option::is_none")]
    pub cpu_limit: Option<i32>,
    /// Which docker tag to pull and start. [Example: v1.0.0]
    #[serde(rename = "dockerTag", skip_serializing_if = "Option::is_none")]
    pub docker_tag: Option<String>,
    /// The QCS service environment from which this stack will request data
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Which branch of the relevant repository to associate with this endpoint. May be used for automatic upgrades on updates to the git branch.
    #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    /// The port on which this component will listen for inbound HTTPS requests
    #[serde(rename = "listenPort", skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<i32>,
    /// Memory allocation in MB required by this component.
    #[serde(rename = "memorySoftLimit", skip_serializing_if = "Option::is_none")]
    pub memory_soft_limit: Option<i32>,
}

impl CertificateProxyComponent {
    pub fn new() -> CertificateProxyComponent {
        CertificateProxyComponent {
            cpu_limit: None,
            docker_tag: None,
            environment: None,
            git_branch: None,
            listen_port: None,
            memory_soft_limit: None,
        }
    }
}
