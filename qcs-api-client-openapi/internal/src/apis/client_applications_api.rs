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
use ::qcs_api_client_common::backoff::{duration_from_response, ExponentialBackoff};
#[cfg(feature = "tracing")]
use qcs_api_client_common::configuration::TokenRefresher;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`check_client_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckClientApplicationError {
    Status404(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_client_application`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientApplicationError {
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_client_applications`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListClientApplicationsError {
    UnknownValue(serde_json::Value),
}

async fn check_client_application_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    check_client_application_request: crate::models::CheckClientApplicationRequest,
) -> Result<crate::models::CheckClientApplicationResponse, Error<CheckClientApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/clientApplications:check",
        local_var_configuration.qcs_config.api_url()
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
                "making check_client_application request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&check_client_application_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<CheckClientApplicationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
            retry_delay: local_var_retry_delay,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Check the requested client application version against the latest and minimum version.
pub async fn check_client_application(
    configuration: &configuration::Configuration,
    check_client_application_request: crate::models::CheckClientApplicationRequest,
) -> Result<crate::models::CheckClientApplicationResponse, Error<CheckClientApplicationError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = check_client_application_inner(
            configuration,
            &mut backoff,
            check_client_application_request.clone(),
        )
        .await;

        match result {
            Ok(result) => return Ok(result),
            Err(Error::ResponseError(response)) => {
                if !refreshed_credentials
                    && matches!(
                        response.status,
                        StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED
                    )
                {
                    configuration.qcs_config.refresh().await?;
                    refreshed_credentials = true;
                    continue;
                } else if let Some(duration) = response.retry_delay {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::ResponseError(response));
            }
            Err(error) => return Err(error),
        }
    }
}
async fn get_client_application_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    client_application_name: &str,
) -> Result<crate::models::ClientApplication, Error<GetClientApplicationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/clientApplications/{clientApplicationName}",
        local_var_configuration.qcs_config.api_url(),
        clientApplicationName = crate::apis::urlencode(client_application_name)
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
                "making get_client_application request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<GetClientApplicationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
            retry_delay: local_var_retry_delay,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get details of a specific Rigetti system component along with its latest and minimum supported versions.
pub async fn get_client_application(
    configuration: &configuration::Configuration,
    client_application_name: &str,
) -> Result<crate::models::ClientApplication, Error<GetClientApplicationError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = get_client_application_inner(
            configuration,
            &mut backoff,
            client_application_name.clone(),
        )
        .await;

        match result {
            Ok(result) => return Ok(result),
            Err(Error::ResponseError(response)) => {
                if !refreshed_credentials
                    && matches!(
                        response.status,
                        StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED
                    )
                {
                    configuration.qcs_config.refresh().await?;
                    refreshed_credentials = true;
                    continue;
                } else if let Some(duration) = response.retry_delay {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::ResponseError(response));
            }
            Err(error) => return Err(error),
        }
    }
}
async fn list_client_applications_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
) -> Result<crate::models::ListClientApplicationsResponse, Error<ListClientApplicationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/clientApplications",
        local_var_configuration.qcs_config.api_url()
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
                "making list_client_applications request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_content = local_var_resp.text().await?;
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<ListClientApplicationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
            retry_delay: local_var_retry_delay,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List supported clients of Rigetti system components along with their latest and minimum supported versions.
pub async fn list_client_applications(
    configuration: &configuration::Configuration,
) -> Result<crate::models::ListClientApplicationsResponse, Error<ListClientApplicationsError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = list_client_applications_inner(configuration, &mut backoff).await;

        match result {
            Ok(result) => return Ok(result),
            Err(Error::ResponseError(response)) => {
                if !refreshed_credentials
                    && matches!(
                        response.status,
                        StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED
                    )
                {
                    configuration.qcs_config.refresh().await?;
                    refreshed_credentials = true;
                    continue;
                } else if let Some(duration) = response.retry_delay {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::ResponseError(response));
            }
            Err(error) => return Err(error),
        }
    }
}
