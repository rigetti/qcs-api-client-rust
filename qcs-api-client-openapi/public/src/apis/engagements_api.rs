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

/// struct for typed errors of method [`create_engagement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEngagementError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    Status422(crate::models::Error),
    Status503(),
    UnknownValue(serde_json::Value),
}

async fn create_engagement_inner(
    configuration: &configuration::Configuration,
    create_engagement_request: crate::models::CreateEngagementRequest,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::EngagementWithCredentials, Error<CreateEngagementError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/engagements", local_var_configuration.base_path);
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

    local_var_req_builder = local_var_req_builder.json(&create_engagement_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateEngagementError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new engagement using the specified parameters.  At least one of the following parameters must be supplied: - **endpointId**: The ID of the endpoint on which to engage. - **quantumProcessorId**: The ID of the quantum processor on which to engage, allowing the     service to select a default endpoint. Ignored if **endpointId** is set.
pub async fn create_engagement(
    configuration: &configuration::Configuration,
    create_engagement_request: crate::models::CreateEngagementRequest,
    x_qcs_account_id: Option<&str>,
    x_qcs_account_type: Option<crate::models::AccountType>,
) -> Result<crate::models::EngagementWithCredentials, Error<CreateEngagementError>> {
    match create_engagement_inner(
        configuration,
        create_engagement_request.clone(),
        x_qcs_account_id.clone(),
        x_qcs_account_type.clone(),
    )
    .await
    {
        Ok(result) => Ok(result),
        Err(err) => match err.status_code() {
            Some(StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED) => {
                configuration.qcs_config.refresh().await?;
                create_engagement_inner(
                    configuration,
                    create_engagement_request,
                    x_qcs_account_id,
                    x_qcs_account_type,
                )
                .await
            }
            _ => Err(err),
        },
    }
}
