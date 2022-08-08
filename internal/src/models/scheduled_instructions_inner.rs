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
pub struct ScheduledInstructionsInner {
    #[serde(rename = "_type")]
    pub _type: String,
    #[serde(rename = "detuning")]
    pub detuning: f32,
    #[serde(rename = "frame")]
    pub frame: String,
    #[serde(rename = "phase")]
    pub phase: f32,
    #[serde(rename = "scale")]
    pub scale: f32,
    #[serde(rename = "time")]
    pub time: f32,
    #[serde(rename = "waveform")]
    pub waveform: String,
    #[serde(rename = "duration")]
    pub duration: f32,
    #[serde(rename = "iq")]
    pub iq: Vec<f32>,
    #[serde(rename = "frequency")]
    pub frequency: f32,
    #[serde(rename = "delta")]
    pub delta: Box<crate::models::Delta>,
    #[serde(rename = "frame_a")]
    pub frame_a: String,
    #[serde(rename = "frame_b")]
    pub frame_b: String,
    #[serde(rename = "filters")]
    pub filters: Vec<String>,
    #[serde(rename = "send_to_host")]
    pub send_to_host: bool,
    #[serde(rename = "message")]
    pub message: i32,
    #[serde(rename = "apply_feedback_when")]
    pub apply_feedback_when: bool,
    #[serde(rename = "attempts")]
    pub attempts: i32,
    #[serde(rename = "feedback_duration")]
    pub feedback_duration: f32,
    #[serde(rename = "feedback_instructions")]
    pub feedback_instructions: Vec<crate::models::FeedbackInstructionsInner>,
    #[serde(rename = "measurement_bit")]
    pub measurement_bit: i32,
    #[serde(rename = "measurement_duration")]
    pub measurement_duration: f32,
    #[serde(rename = "measurement_instructions")]
    pub measurement_instructions: Vec<crate::models::FeedbackInstructionsInner>,
}

impl ScheduledInstructionsInner {
    pub fn new(_type: String, detuning: f32, frame: String, phase: f32, scale: f32, time: f32, waveform: String, duration: f32, iq: Vec<f32>, frequency: f32, delta: crate::models::Delta, frame_a: String, frame_b: String, filters: Vec<String>, send_to_host: bool, message: i32, apply_feedback_when: bool, attempts: i32, feedback_duration: f32, feedback_instructions: Vec<crate::models::FeedbackInstructionsInner>, measurement_bit: i32, measurement_duration: f32, measurement_instructions: Vec<crate::models::FeedbackInstructionsInner>) -> ScheduledInstructionsInner {
        ScheduledInstructionsInner {
            _type,
            detuning,
            frame,
            phase,
            scale,
            time,
            waveform,
            duration,
            iq,
            frequency,
            delta: Box::new(delta),
            frame_a,
            frame_b,
            filters,
            send_to_host,
            message,
            apply_feedback_when,
            attempts,
            feedback_duration,
            feedback_instructions,
            measurement_bit,
            measurement_duration,
            measurement_instructions,
        }
    }
}


