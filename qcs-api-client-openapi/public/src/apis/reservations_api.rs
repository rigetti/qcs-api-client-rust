// Copyright 2021 Rigetti Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

/// struct for typed errors of method [`create_reservation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateReservationError {
    Status401(crate::models::Error),
    Status402(crate::models::Error),
    Status403(crate::models::Error),
    Status409(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_reservation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteReservationError {
    Status401(crate::models::Error),
    Status403(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`find_available_reservations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindAvailableReservationsError {
    Status401(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_group_reservations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGroupReservationsError {
    Status401(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_reservations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListReservationsError {
    Status401(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

async fn create_reservation_inner(
    configuration: &configuration::Configuration,
    create_reservation_request: crate::models::CreateReservationRequest,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::Reservation, Error<CreateReservationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/reservations", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(local_var_param_value) = x_qcs_account_id {
        local_var_req_builder =
            local_var_req_builder.header("x-qcs-account-id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_qcs_account_type {
        local_var_req_builder =
            local_var_req_builder.header("x-qcs-account-type", local_var_param_value.to_string());
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&create_reservation_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateReservationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new reservation.  The following precedence applies when specifying the reservation subject account ID and type: * request body `accountId` field, or if unset then `X-QCS-ACCOUNT-ID` header, or if unset then requesting user's ID. * request body `accountType` field, or if unset then `X-QCS-ACCOUNT-TYPE` header, or if unset then \"user\" type.
pub async fn create_reservation(
    configuration: &configuration::Configuration,
    create_reservation_request: crate::models::CreateReservationRequest,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::Reservation, Error<CreateReservationError>> {
    match create_reservation_inner(
        configuration,
        create_reservation_request.clone(),
        x_qcs_account_id.clone(),
        x_qcs_account_type.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                create_reservation_inner(
                    configuration,
                    create_reservation_request,
                    x_qcs_account_id,
                    x_qcs_account_type,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn delete_reservation_inner(
    configuration: &configuration::Configuration,
    reservation_id: i32,
) -> Result<crate::models::Reservation, Error<DeleteReservationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/reservations/{reservationId}",
        local_var_configuration.base_path,
        reservationId = reservation_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteReservationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel an existing reservation for the user.
pub async fn delete_reservation(
    configuration: &configuration::Configuration,
    reservation_id: i32,
) -> Result<crate::models::Reservation, Error<DeleteReservationError>> {
    match delete_reservation_inner(configuration, reservation_id.clone()).await {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                delete_reservation_inner(configuration, reservation_id).await
            }
            _ => Err(err),
        },
    }
}
async fn find_available_reservations_inner(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    start_time_from: String,
    duration: &str,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<crate::models::FindAvailableReservationsResponse, Error<FindAvailableReservationsError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/reservations:findAvailable",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    local_var_req_builder =
        local_var_req_builder.query(&[("quantumProcessorId", &quantum_processor_id.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("startTimeFrom", &start_time_from.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("duration", &duration.to_string())]);

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
        let local_var_entity: Option<FindAvailableReservationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List currently available reservations on the requested Rigetti quantum computer.
pub async fn find_available_reservations(
    configuration: &configuration::Configuration,
    quantum_processor_id: &str,
    start_time_from: String,
    duration: &str,
    page_size: Option<i32>,
    page_token: Option<&str>,
) -> Result<crate::models::FindAvailableReservationsResponse, Error<FindAvailableReservationsError>>
{
    match find_available_reservations_inner(
        configuration,
        quantum_processor_id.clone(),
        start_time_from.clone(),
        duration.clone(),
        page_size.clone(),
        page_token.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                find_available_reservations_inner(
                    configuration,
                    quantum_processor_id,
                    start_time_from,
                    duration,
                    page_size,
                    page_token,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn list_group_reservations_inner(
    configuration: &configuration::Configuration,
    group_name: &str,
    filter: Option<&str>,
    order: Option<&str>,
    page_size: Option<i32>,
    page_token: Option<&str>,
    show_deleted: Option<&str>,
) -> Result<crate::models::ListReservationsResponse, Error<ListGroupReservationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/groups/{groupName}/reservations",
        local_var_configuration.base_path,
        groupName = crate::apis::urlencode(group_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder =
            local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_deleted {
        local_var_req_builder =
            local_var_req_builder.query(&[("showDeleted", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListGroupReservationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List existing reservations for the requested group.  Available filter fields include:  * `startTime` - timestamp * `endTime` - timestamp * `createdTime` - timestamp * `price` - integer * `quantumProcessorId` - string  Available order fields include:  * `startTime` - timestamp * `endTime` - timestamp * `createdTime` - timestamp * `price` - integer
pub async fn list_group_reservations(
    configuration: &configuration::Configuration,
    group_name: &str,
    filter: Option<&str>,
    order: Option<&str>,
    page_size: Option<i32>,
    page_token: Option<&str>,
    show_deleted: Option<&str>,
) -> Result<crate::models::ListReservationsResponse, Error<ListGroupReservationsError>> {
    match list_group_reservations_inner(
        configuration,
        group_name.clone(),
        filter.clone(),
        order.clone(),
        page_size.clone(),
        page_token.clone(),
        show_deleted.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                list_group_reservations_inner(
                    configuration,
                    group_name,
                    filter,
                    order,
                    page_size,
                    page_token,
                    show_deleted,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
async fn list_reservations_inner(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
    order: Option<&str>,
    page_size: Option<i32>,
    page_token: Option<&str>,
    show_deleted: Option<&str>,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::ListReservationsResponse, Error<ListReservationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/reservations", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order {
        local_var_req_builder =
            local_var_req_builder.query(&[("order", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = show_deleted {
        local_var_req_builder =
            local_var_req_builder.query(&[("showDeleted", &local_var_str.to_string())]);
    }
    if let Some(local_var_param_value) = x_qcs_account_id {
        local_var_req_builder =
            local_var_req_builder.header("x-qcs-account-id", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_qcs_account_type {
        local_var_req_builder =
            local_var_req_builder.header("x-qcs-account-type", local_var_param_value.to_string());
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
        let local_var_entity: Option<ListReservationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List existing reservations for the authenticated user, or a target user when specifying `X-QCS-ACCOUNT-ID` and `X-QCS-ACCOUNT-TYPE` headers.  Available filter fields include:  * `startTime` - timestamp * `endTime` - timestamp * `createdTime` - timestamp * `price` - integer * `cancelled` - boolean (deprecated, use `showDeleted` parameter) * `quantumProcessorId` - string  Available order fields include:  * `startTime` - timestamp * `endTime` - timestamp * `createdTime` - timestamp * `price` - integer
pub async fn list_reservations(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
    order: Option<&str>,
    page_size: Option<i32>,
    page_token: Option<&str>,
    show_deleted: Option<&str>,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::ListReservationsResponse, Error<ListReservationsError>> {
    match list_reservations_inner(
        configuration,
        filter.clone(),
        order.clone(),
        page_size.clone(),
        page_token.clone(),
        show_deleted.clone(),
        x_qcs_account_id.clone(),
        x_qcs_account_type.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                list_reservations_inner(
                    configuration,
                    filter,
                    order,
                    page_size,
                    page_token,
                    show_deleted,
                    x_qcs_account_id,
                    x_qcs_account_type,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
