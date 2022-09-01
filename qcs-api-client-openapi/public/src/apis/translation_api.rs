/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ResponseContent;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`get_quilt_calibrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQuiltCalibrationsError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`translate_native_quil_to_encrypted_binary`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslateNativeQuilToEncryptedBinaryError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

async fn get_quilt_calibrations_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<crate::models::GetQuiltCalibrationsResponse, Error<GetQuiltCalibrationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/quantumProcessors/{quantumProcessorId}/quiltCalibrations",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetQuiltCalibrationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the calibration data used for client-side Quilt generation.
pub async fn get_quilt_calibrations(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<crate::models::GetQuiltCalibrationsResponse, Error<GetQuiltCalibrationsError>> {
    match get_quilt_calibrations_inner(configuration, quantum_processor_id.clone()).await {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                get_quilt_calibrations_inner(configuration, quantum_processor_id).await
            }
            _ => Err(err),
        },
    }
}
async fn translate_native_quil_to_encrypted_binary_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    translate_native_quil_to_encrypted_binary_request: crate::models::TranslateNativeQuilToEncryptedBinaryRequest,
) -> Result<
    crate::models::TranslateNativeQuilToEncryptedBinaryResponse,
    Error<TranslateNativeQuilToEncryptedBinaryError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/quantumProcessors/{quantumProcessorId}:translateNativeQuilToEncryptedBinary",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder =
        local_var_req_builder.json(&translate_native_quil_to_encrypted_binary_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TranslateNativeQuilToEncryptedBinaryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Compile Rigetti-native Quil code to encrypted binary form, ready for execution on a Rigetti Quantum Processor.
pub async fn translate_native_quil_to_encrypted_binary(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    translate_native_quil_to_encrypted_binary_request: crate::models::TranslateNativeQuilToEncryptedBinaryRequest,
) -> Result<
    crate::models::TranslateNativeQuilToEncryptedBinaryResponse,
    Error<TranslateNativeQuilToEncryptedBinaryError>,
> {
    match translate_native_quil_to_encrypted_binary_inner(
        configuration,
        quantum_processor_id.clone(),
        translate_native_quil_to_encrypted_binary_request.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                translate_native_quil_to_encrypted_binary_inner(
                    configuration,
                    quantum_processor_id,
                    translate_native_quil_to_encrypted_binary_request,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
