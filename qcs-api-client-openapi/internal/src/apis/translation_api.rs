/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://github.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://github.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ResponseContent;
#[cfg(feature = "tracing")]
use qcs_api_client_common::configuration::TokenRefresher;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`build_qpu_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildQpuSettingsError {
    Status400(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_qpu_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQpuSettingsError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_quilt_calibrations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetQuiltCalibrationsError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_qpu_settings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutQpuSettingsError {
    Status400(crate::models::Error),
    Status422(crate::models::Error),
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

async fn build_qpu_settings_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<crate::models::QpuSettings, Error<BuildQpuSettingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/quantumProcessors/{quantumProcessorId}/qpuSettings:build",
        local_var_configuration.qcs_config.api_url(),
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    #[cfg(feature = "tracing")]
    {
        // Ignore parsing errors if the URL is invalid for some reason.
        // If it is invalid, it will turn up as an error later when actually making the request.
        let local_var_do_tracing =
            local_var_uri_str
                .parse::<::url::Url>()
                .ok()
                .map_or(true, |url| {
                    configuration
                        .qcs_config
                        .should_trace(&::urlpattern::UrlPatternMatchInput::Url(url))
                });

        if local_var_do_tracing {
            ::tracing::debug!(
                url=%local_var_uri_str,
                method="POST",
                "making build_qpu_settings request",
            );
        }
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
        let local_var_entity: Option<BuildQpuSettingsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Trigger a rebuild of the QPUSettings used in translation.
pub async fn build_qpu_settings(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<crate::models::QpuSettings, Error<BuildQpuSettingsError>> {
    match build_qpu_settings_inner(configuration, quantum_processor_id.clone()).await {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                build_qpu_settings_inner(configuration, quantum_processor_id).await
            }
            _ => Err(err),
        },
    }
}
async fn get_qpu_settings_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    timestamp: Option<&str>,
) -> Result<crate::models::QpuSettings, Error<GetQpuSettingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/quantumProcessors/{quantumProcessorId}/qpuSettings",
        local_var_configuration.qcs_config.api_url(),
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    #[cfg(feature = "tracing")]
    {
        // Ignore parsing errors if the URL is invalid for some reason.
        // If it is invalid, it will turn up as an error later when actually making the request.
        let local_var_do_tracing =
            local_var_uri_str
                .parse::<::url::Url>()
                .ok()
                .map_or(true, |url| {
                    configuration
                        .qcs_config
                        .should_trace(&::urlpattern::UrlPatternMatchInput::Url(url))
                });

        if local_var_do_tracing {
            ::tracing::debug!(
                url=%local_var_uri_str,
                method="GET",
                "making get_qpu_settings request",
            );
        }
    }

    if let Some(ref local_var_str) = timestamp {
        local_var_req_builder =
            local_var_req_builder.query(&[("timestamp", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetQpuSettingsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the QPUSettings used in translation in adapted JSON format.  Note: not all keys used in the QPUSettings object are strings, and thus are not JSON-compliant. These have been coerced to strings for human review, but this can not be re-hydrated into a QPUSettings object without a reverse transformation.
pub async fn get_qpu_settings(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    timestamp: Option<&str>,
) -> Result<crate::models::QpuSettings, Error<GetQpuSettingsError>> {
    match get_qpu_settings_inner(
        configuration,
        quantum_processor_id.clone(),
        timestamp.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                get_qpu_settings_inner(configuration, quantum_processor_id, timestamp).await
            }
            _ => Err(err),
        },
    }
}
async fn get_quilt_calibrations_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<crate::models::GetQuiltCalibrationsResponse, Error<GetQuiltCalibrationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/quantumProcessors/{quantumProcessorId}/quiltCalibrations",
        local_var_configuration.qcs_config.api_url(),
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    #[cfg(feature = "tracing")]
    {
        // Ignore parsing errors if the URL is invalid for some reason.
        // If it is invalid, it will turn up as an error later when actually making the request.
        let local_var_do_tracing =
            local_var_uri_str
                .parse::<::url::Url>()
                .ok()
                .map_or(true, |url| {
                    configuration
                        .qcs_config
                        .should_trace(&::urlpattern::UrlPatternMatchInput::Url(url))
                });

        if local_var_do_tracing {
            ::tracing::debug!(
                url=%local_var_uri_str,
                method="GET",
                "making get_quilt_calibrations request",
            );
        }
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
async fn put_qpu_settings_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    put_qpu_settings_request: crate::models::PutQpuSettingsRequest,
) -> Result<crate::models::QpuSettings, Error<PutQpuSettingsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/quantumProcessors/{quantumProcessorId}/qpuSettings",
        local_var_configuration.qcs_config.api_url(),
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    #[cfg(feature = "tracing")]
    {
        // Ignore parsing errors if the URL is invalid for some reason.
        // If it is invalid, it will turn up as an error later when actually making the request.
        let local_var_do_tracing =
            local_var_uri_str
                .parse::<::url::Url>()
                .ok()
                .map_or(true, |url| {
                    configuration
                        .qcs_config
                        .should_trace(&::urlpattern::UrlPatternMatchInput::Url(url))
                });

        if local_var_do_tracing {
            ::tracing::debug!(
                url=%local_var_uri_str,
                method="PUT",
                "making put_qpu_settings request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&put_qpu_settings_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutQpuSettingsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Publish a new version of QPUSettings for a quantum processor.
pub async fn put_qpu_settings(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    put_qpu_settings_request: crate::models::PutQpuSettingsRequest,
) -> Result<crate::models::QpuSettings, Error<PutQpuSettingsError>> {
    match put_qpu_settings_inner(
        configuration,
        quantum_processor_id.clone(),
        put_qpu_settings_request.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                put_qpu_settings_inner(
                    configuration,
                    quantum_processor_id,
                    put_qpu_settings_request,
                )
                .await
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
        local_var_configuration.qcs_config.api_url(),
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    #[cfg(feature = "tracing")]
    {
        // Ignore parsing errors if the URL is invalid for some reason.
        // If it is invalid, it will turn up as an error later when actually making the request.
        let local_var_do_tracing =
            local_var_uri_str
                .parse::<::url::Url>()
                .ok()
                .map_or(true, |url| {
                    configuration
                        .qcs_config
                        .should_trace(&::urlpattern::UrlPatternMatchInput::Url(url))
                });

        if local_var_do_tracing {
            ::tracing::debug!(
                url=%local_var_uri_str,
                method="POST",
                "making translate_native_quil_to_encrypted_binary request",
            );
        }
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
