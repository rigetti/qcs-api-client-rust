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
pub struct ControllerComponent {
    /// Define the controller's relationship to hardware controls
    #[serde(rename = "backend", skip_serializing_if = "Option::is_none")]
    pub backend: Option<Box<crate::models::ExecutionBackend>>,
    /// Command line arguments to append to the default values in the controller start command
    #[serde(rename = "commandLineArgs", skip_serializing_if = "Option::is_none")]
    pub command_line_args: Option<Vec<String>>,
    /// CPU Allocation, in MHz, required by this component. Whether it is a hard or soft limit is specified by the component itself. By default, it is a soft limit, and components are allowed to burst above when there is unused capacity.
    #[serde(rename = "cpuLimit", skip_serializing_if = "Option::is_none")]
    pub cpu_limit: Option<i32>,
    /// Which docker tag to pull and start. [Example: v1.0.0] Does not support the `latest` tag.
    #[serde(rename = "dockerTag", skip_serializing_if = "Option::is_none")]
    pub docker_tag: Option<String>,
    /// Whether to send errors and data to Sentry
    #[serde(rename = "enableSentry", skip_serializing_if = "Option::is_none")]
    pub enable_sentry: Option<bool>,
    /// Setting to false will disable TLS on connections. Changing this value is a breaking change for clients of the endpoint.
    #[serde(rename = "enforceTls", skip_serializing_if = "Option::is_none")]
    pub enforce_tls: Option<bool>,
    /// The QCS service environment from which this stack will request data
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Which fridge wiring architecture this Controller Service should implement
    #[serde(rename = "fridgeId", skip_serializing_if = "Option::is_none")]
    pub fridge_id: Option<String>,
    /// Which branch of the relevant repository to associate with this endpoint. May be used for automatic upgrades on updates to the git branch.
    #[serde(rename = "gitBranch", skip_serializing_if = "Option::is_none")]
    pub git_branch: Option<String>,
    /// Determines whether events are produced to Kafka, logged to stdout, or ignored
    #[serde(
        rename = "kafkaEventProducerTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub kafka_event_producer_types: Option<Vec<crate::models::KafkaEventProducer>>,
    /// Fixed network ports keyed on port name
    #[serde(rename = "listenPorts", skip_serializing_if = "Option::is_none")]
    pub listen_ports: Option<::std::collections::HashMap<String, i32>>,
    /// Memory allocation in MB required by this component.
    #[serde(rename = "memorySoftLimit", skip_serializing_if = "Option::is_none")]
    pub memory_soft_limit: Option<i32>,
    /// Strategy used when deciding which jobs take priority and how to execute them (parallel vs. sequential).
    #[serde(rename = "queuePolicyType", skip_serializing_if = "Option::is_none")]
    pub queue_policy_type: Option<Box<crate::models::QueuePolicyType>>,
    /// Select the data source from which the endpoint's startup configuration should be retrieved
    #[serde(
        rename = "startupConfigurationSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_configuration_source: Option<Box<crate::models::StartupConfigurationSource>>,
    /// Whether to pass through access to USB instruments with host VISA configuration
    #[serde(rename = "visaPassthrough", skip_serializing_if = "Option::is_none")]
    pub visa_passthrough: Option<bool>,
}

impl ControllerComponent {
    pub fn new() -> ControllerComponent {
        ControllerComponent {
            backend: None,
            command_line_args: None,
            cpu_limit: None,
            docker_tag: None,
            enable_sentry: None,
            enforce_tls: None,
            environment: None,
            fridge_id: None,
            git_branch: None,
            kafka_event_producer_types: None,
            listen_ports: None,
            memory_soft_limit: None,
            queue_policy_type: None,
            startup_configuration_source: None,
            visa_passthrough: None,
        }
    }
}
