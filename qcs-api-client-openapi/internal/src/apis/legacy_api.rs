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

/// struct for typed errors of method [`internal_delete_legacy_deployed_rack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalDeleteLegacyDeployedRackError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_delete_legacy_quantum_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalDeleteLegacyQuantumProcessorError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_get_legacy_deployed_rack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalGetLegacyDeployedRackError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_get_legacy_lattice`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalGetLegacyLatticeError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_get_legacy_quantum_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalGetLegacyQuantumProcessorError {
    Status404(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_list_legacy_lattices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalListLegacyLatticesError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_list_legacy_quantum_processors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalListLegacyQuantumProcessorsError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_put_legacy_deployed_rack`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalPutLegacyDeployedRackError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_put_legacy_quantum_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalPutLegacyQuantumProcessorError {
    Status400(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_update_legacy_quantum_processor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalUpdateLegacyQuantumProcessorError {
    Status400(crate::models::Error),
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

async fn internal_delete_legacy_deployed_rack_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<serde_json::Value, Error<InternalDeleteLegacyDeployedRackError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}/deployedRack",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<InternalDeleteLegacyDeployedRackError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the Deployed Rack for a Quantum Processor
pub async fn internal_delete_legacy_deployed_rack(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<serde_json::Value, Error<InternalDeleteLegacyDeployedRackError>> {
    match internal_delete_legacy_deployed_rack_inner(configuration, quantum_processor_id.clone())
        .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_delete_legacy_deployed_rack_inner(configuration, quantum_processor_id)
                    .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_delete_legacy_quantum_processor_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<serde_json::Value, Error<InternalDeleteLegacyQuantumProcessorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<InternalDeleteLegacyQuantumProcessorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete a legacy (Forest Server) Quantum Processor.
pub async fn internal_delete_legacy_quantum_processor(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
) -> Result<serde_json::Value, Error<InternalDeleteLegacyQuantumProcessorError>> {
    match internal_delete_legacy_quantum_processor_inner(
        configuration,
        quantum_processor_id.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_delete_legacy_quantum_processor_inner(configuration, quantum_processor_id)
                    .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_get_legacy_deployed_rack_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mock: Option<bool>,
) -> Result<crate::models::DeployedRack, Error<InternalGetLegacyDeployedRackError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}/deployedRack",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = mock {
        local_var_req_builder =
            local_var_req_builder.query(&[("mock", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<InternalGetLegacyDeployedRackError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the Deployed Rack for a Quantum Processor.
pub async fn internal_get_legacy_deployed_rack(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mock: Option<bool>,
) -> Result<crate::models::DeployedRack, Error<InternalGetLegacyDeployedRackError>> {
    match internal_get_legacy_deployed_rack_inner(
        configuration,
        quantum_processor_id.clone(),
        mock.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_get_legacy_deployed_rack_inner(configuration, quantum_processor_id, mock)
                    .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_get_legacy_lattice_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mask_specifications_to_isa: Option<bool>,
) -> Result<crate::models::LegacyLattice, Error<InternalGetLegacyLatticeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/lattices/{quantumProcessorId}",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = mask_specifications_to_isa {
        local_var_req_builder = local_var_req_builder
            .query(&[("mask_specifications_to_isa", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<InternalGetLegacyLatticeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a legacy lattice by name. Provided for Forest Server deprecation only. Note that it makes the key assumption that the legacy device/quantum processor has exactly one full-chip lattice by the same name.  In practice, this retrieves the LegacyQuantumProcessor and then converts it to the lattice schema.
pub async fn internal_get_legacy_lattice(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mask_specifications_to_isa: Option<bool>,
) -> Result<crate::models::LegacyLattice, Error<InternalGetLegacyLatticeError>> {
    match internal_get_legacy_lattice_inner(
        configuration,
        quantum_processor_id.clone(),
        mask_specifications_to_isa.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_get_legacy_lattice_inner(
                    configuration,
                    quantum_processor_id,
                    mask_specifications_to_isa,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_get_legacy_quantum_processor_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mask_specifications_to_isa: Option<bool>,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalGetLegacyQuantumProcessorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = mask_specifications_to_isa {
        local_var_req_builder = local_var_req_builder
            .query(&[("mask_specifications_to_isa", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<InternalGetLegacyQuantumProcessorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve the legacy Forest Server configuration of a quantum processor.
pub async fn internal_get_legacy_quantum_processor(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    mask_specifications_to_isa: Option<bool>,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalGetLegacyQuantumProcessorError>> {
    match internal_get_legacy_quantum_processor_inner(
        configuration,
        quantum_processor_id.clone(),
        mask_specifications_to_isa.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_get_legacy_quantum_processor_inner(
                    configuration,
                    quantum_processor_id,
                    mask_specifications_to_isa,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_list_legacy_lattices_inner(
    configuration: &configuration::Configuration,
    mask_specifications_to_isa: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<crate::models::InternalListLegacyLatticesResponse, Error<InternalListLegacyLatticesError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/lattices",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = mask_specifications_to_isa {
        local_var_req_builder = local_var_req_builder
            .query(&[("mask_specifications_to_isa", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<InternalListLegacyLatticesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve all legacy (Forest Server) lattices available to the user.
pub async fn internal_list_legacy_lattices(
    configuration: &configuration::Configuration,
    mask_specifications_to_isa: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<crate::models::InternalListLegacyLatticesResponse, Error<InternalListLegacyLatticesError>>
{
    match internal_list_legacy_lattices_inner(
        configuration,
        mask_specifications_to_isa.clone(),
        page_size.clone(),
        page_token.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_list_legacy_lattices_inner(
                    configuration,
                    mask_specifications_to_isa,
                    page_size,
                    page_token,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_list_legacy_quantum_processors_inner(
    configuration: &configuration::Configuration,
    mask_specifications_to_isa: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<
    crate::models::InternalListLegacyQuantumProcessorsResponse,
    Error<InternalListLegacyQuantumProcessorsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = mask_specifications_to_isa {
        local_var_req_builder = local_var_req_builder
            .query(&[("mask_specifications_to_isa", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<InternalListLegacyQuantumProcessorsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve all legacy (Forest Server) Quantum Processors available to the user.
pub async fn internal_list_legacy_quantum_processors(
    configuration: &configuration::Configuration,
    mask_specifications_to_isa: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<
    crate::models::InternalListLegacyQuantumProcessorsResponse,
    Error<InternalListLegacyQuantumProcessorsError>,
> {
    match internal_list_legacy_quantum_processors_inner(
        configuration,
        mask_specifications_to_isa.clone(),
        page_size.clone(),
        page_token.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_list_legacy_quantum_processors_inner(
                    configuration,
                    mask_specifications_to_isa,
                    page_size,
                    page_token,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_put_legacy_deployed_rack_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_put_legacy_deployed_rack_request: crate::models::InternalPutLegacyDeployedRackRequest,
) -> Result<crate::models::DeployedRack, Error<InternalPutLegacyDeployedRackError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}/deployedRack",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&internal_put_legacy_deployed_rack_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InternalPutLegacyDeployedRackError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create or update the Deployed Rack for a Quantum Processor.
pub async fn internal_put_legacy_deployed_rack(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_put_legacy_deployed_rack_request: crate::models::InternalPutLegacyDeployedRackRequest,
) -> Result<crate::models::DeployedRack, Error<InternalPutLegacyDeployedRackError>> {
    match internal_put_legacy_deployed_rack_inner(
        configuration,
        quantum_processor_id.clone(),
        internal_put_legacy_deployed_rack_request.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_put_legacy_deployed_rack_inner(
                    configuration,
                    quantum_processor_id,
                    internal_put_legacy_deployed_rack_request,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_put_legacy_quantum_processor_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_put_legacy_quantum_processor_request: crate::models::InternalPutLegacyQuantumProcessorRequest,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalPutLegacyQuantumProcessorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder =
        local_var_req_builder.json(&internal_put_legacy_quantum_processor_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InternalPutLegacyQuantumProcessorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create or replace the legacy Forest Server configuration of a device.
pub async fn internal_put_legacy_quantum_processor(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_put_legacy_quantum_processor_request: crate::models::InternalPutLegacyQuantumProcessorRequest,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalPutLegacyQuantumProcessorError>> {
    match internal_put_legacy_quantum_processor_inner(
        configuration,
        quantum_processor_id.clone(),
        internal_put_legacy_quantum_processor_request.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_put_legacy_quantum_processor_inner(
                    configuration,
                    quantum_processor_id,
                    internal_put_legacy_quantum_processor_request,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn internal_update_legacy_quantum_processor_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_update_legacy_quantum_processor_request: crate::models::InternalUpdateLegacyQuantumProcessorRequest,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalUpdateLegacyQuantumProcessorError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/legacy/quantumProcessors/{quantumProcessorId}",
        local_var_configuration.base_path,
        quantumProcessorId = crate::apis::urlencode(quantum_processor_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder =
        local_var_req_builder.json(&internal_update_legacy_quantum_processor_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<InternalUpdateLegacyQuantumProcessorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Patch an existing LegacyQuantumProcessor.
pub async fn internal_update_legacy_quantum_processor(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    internal_update_legacy_quantum_processor_request: crate::models::InternalUpdateLegacyQuantumProcessorRequest,
) -> Result<crate::models::LegacyQuantumProcessor, Error<InternalUpdateLegacyQuantumProcessorError>>
{
    match internal_update_legacy_quantum_processor_inner(
        configuration,
        quantum_processor_id.clone(),
        internal_update_legacy_quantum_processor_request.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                internal_update_legacy_quantum_processor_inner(
                    configuration,
                    quantum_processor_id,
                    internal_update_legacy_quantum_processor_request,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
