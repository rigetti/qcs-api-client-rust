// Copyright 2022 Rigetti Computing
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
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://github.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://github.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::apis::ResponseContent;
use ::qcs_api_client_common::backoff::{
    duration_from_io_error, duration_from_reqwest_error, duration_from_response, ExponentialBackoff,
};
#[cfg(feature = "tracing")]
use qcs_api_client_common::configuration::TokenRefresher;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`auth_email_password_reset_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthEmailPasswordResetTokenError {
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthGetUserError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_reset_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthResetPasswordError {
    Status401(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_reset_password_with_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthResetPasswordWithTokenError {
    Status404(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

async fn auth_email_password_reset_token_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    auth_email_password_reset_token_request: Option<
        crate::models::AuthEmailPasswordResetTokenRequest,
    >,
) -> Result<(), Error<AuthEmailPasswordResetTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/auth:emailPasswordResetToken",
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
                "making auth_email_password_reset_token request",
            );
        }
    }

    // Use the QCS Bearer token if a client OAuthSession is present,
    // but do not require one when the security schema says it is optional.
    {
        use qcs_api_client_common::configuration::TokenError;

        #[allow(
            clippy::nonminimal_bool,
            clippy::eq_op,
            reason = "Logic must be done at runtime since it cannot be handled by the mustache template engine."
        )]
        let is_jwt_bearer_optional: bool = false;

        let token = local_var_configuration
            .qcs_config
            .get_bearer_access_token()
            .await;

        if is_jwt_bearer_optional && matches!(token, Err(TokenError::NoCredentials)) {
            // the client is configured without any OAuthSession, but this call does not require one.
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "No client credentials found, but this call does not require authentication."
            );
        } else {
            local_var_req_builder = local_var_req_builder.bearer_auth(token?);
        }
    }

    local_var_req_builder = local_var_req_builder.json(&auth_email_password_reset_token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<AuthEmailPasswordResetTokenError> =
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

/// Send a password reset link to the provided email address, if that email matches a registered user.
pub async fn auth_email_password_reset_token(
    configuration: &configuration::Configuration,
    auth_email_password_reset_token_request: Option<
        crate::models::AuthEmailPasswordResetTokenRequest,
    >,
) -> Result<(), Error<AuthEmailPasswordResetTokenError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    let method = reqwest::Method::POST;
    loop {
        let result = auth_email_password_reset_token_inner(
            configuration,
            &mut backoff,
            auth_email_password_reset_token_request.clone(),
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
            Err(Error::Reqwest(error)) => {
                if let Some(duration) = duration_from_reqwest_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Reqwest(error));
            }
            Err(Error::Io(error)) => {
                if let Some(duration) = duration_from_io_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Io(error));
            }
            Err(error) => return Err(error),
        }
    }
}
async fn auth_get_user_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
) -> Result<crate::models::User, Error<AuthGetUserError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/auth:getUser",
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
                "making auth_get_user request",
            );
        }
    }

    // Use the QCS Bearer token if a client OAuthSession is present,
    // but do not require one when the security schema says it is optional.
    {
        use qcs_api_client_common::configuration::TokenError;

        #[allow(
            clippy::nonminimal_bool,
            clippy::eq_op,
            reason = "Logic must be done at runtime since it cannot be handled by the mustache template engine."
        )]
        let is_jwt_bearer_optional: bool = false || "JWTBearer" == "JWTBearerOptional";

        let token = local_var_configuration
            .qcs_config
            .get_bearer_access_token()
            .await;

        if is_jwt_bearer_optional && matches!(token, Err(TokenError::NoCredentials)) {
            // the client is configured without any OAuthSession, but this call does not require one.
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "No client credentials found, but this call does not require authentication."
            );
        } else {
            local_var_req_builder = local_var_req_builder.bearer_auth(token?);
        }
    }

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
        let local_var_entity: Option<AuthGetUserError> =
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

/// Retrieve the profile of the authenticated user.
pub async fn auth_get_user(
    configuration: &configuration::Configuration,
) -> Result<crate::models::User, Error<AuthGetUserError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    let method = reqwest::Method::GET;
    loop {
        let result = auth_get_user_inner(configuration, &mut backoff).await;

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
            Err(Error::Reqwest(error)) => {
                if let Some(duration) = duration_from_reqwest_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Reqwest(error));
            }
            Err(Error::Io(error)) => {
                if let Some(duration) = duration_from_io_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Io(error));
            }
            Err(error) => return Err(error),
        }
    }
}
async fn auth_reset_password_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    auth_reset_password_request: crate::models::AuthResetPasswordRequest,
) -> Result<(), Error<AuthResetPasswordError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/auth:resetPassword",
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
                "making auth_reset_password request",
            );
        }
    }

    // Use the QCS Bearer token if a client OAuthSession is present,
    // but do not require one when the security schema says it is optional.
    {
        use qcs_api_client_common::configuration::TokenError;

        #[allow(
            clippy::nonminimal_bool,
            clippy::eq_op,
            reason = "Logic must be done at runtime since it cannot be handled by the mustache template engine."
        )]
        let is_jwt_bearer_optional: bool = false || "JWTBearer" == "JWTBearerOptional";

        let token = local_var_configuration
            .qcs_config
            .get_bearer_access_token()
            .await;

        if is_jwt_bearer_optional && matches!(token, Err(TokenError::NoCredentials)) {
            // the client is configured without any OAuthSession, but this call does not require one.
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "No client credentials found, but this call does not require authentication."
            );
        } else {
            local_var_req_builder = local_var_req_builder.bearer_auth(token?);
        }
    }

    local_var_req_builder = local_var_req_builder.json(&auth_reset_password_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<AuthResetPasswordError> =
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

/// Reset the password using the user's existing password. Note, this is an authenticated route.
pub async fn auth_reset_password(
    configuration: &configuration::Configuration,
    auth_reset_password_request: crate::models::AuthResetPasswordRequest,
) -> Result<(), Error<AuthResetPasswordError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    let method = reqwest::Method::POST;
    loop {
        let result = auth_reset_password_inner(
            configuration,
            &mut backoff,
            auth_reset_password_request.clone(),
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
            Err(Error::Reqwest(error)) => {
                if let Some(duration) = duration_from_reqwest_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Reqwest(error));
            }
            Err(Error::Io(error)) => {
                if let Some(duration) = duration_from_io_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Io(error));
            }
            Err(error) => return Err(error),
        }
    }
}
async fn auth_reset_password_with_token_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    auth_reset_password_with_token_request: crate::models::AuthResetPasswordWithTokenRequest,
) -> Result<(), Error<AuthResetPasswordWithTokenError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/auth:resetPasswordWithToken",
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
                "making auth_reset_password_with_token request",
            );
        }
    }

    // Use the QCS Bearer token if a client OAuthSession is present,
    // but do not require one when the security schema says it is optional.
    {
        use qcs_api_client_common::configuration::TokenError;

        #[allow(
            clippy::nonminimal_bool,
            clippy::eq_op,
            reason = "Logic must be done at runtime since it cannot be handled by the mustache template engine."
        )]
        let is_jwt_bearer_optional: bool = false;

        let token = local_var_configuration
            .qcs_config
            .get_bearer_access_token()
            .await;

        if is_jwt_bearer_optional && matches!(token, Err(TokenError::NoCredentials)) {
            // the client is configured without any OAuthSession, but this call does not require one.
            #[cfg(feature = "tracing")]
            tracing::debug!(
                "No client credentials found, but this call does not require authentication."
            );
        } else {
            local_var_req_builder = local_var_req_builder.bearer_auth(token?);
        }
    }

    local_var_req_builder = local_var_req_builder.json(&auth_reset_password_with_token_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_retry_delay =
            duration_from_response(local_var_resp.status(), local_var_resp.headers(), backoff);
        let local_var_content = local_var_resp.text().await?;
        let local_var_entity: Option<AuthResetPasswordWithTokenError> =
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

/// Complete the forgot password flow, resetting the new password in exchange for an emailed token.
pub async fn auth_reset_password_with_token(
    configuration: &configuration::Configuration,
    auth_reset_password_with_token_request: crate::models::AuthResetPasswordWithTokenRequest,
) -> Result<(), Error<AuthResetPasswordWithTokenError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    let method = reqwest::Method::POST;
    loop {
        let result = auth_reset_password_with_token_inner(
            configuration,
            &mut backoff,
            auth_reset_password_with_token_request.clone(),
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
            Err(Error::Reqwest(error)) => {
                if let Some(duration) = duration_from_reqwest_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Reqwest(error));
            }
            Err(Error::Io(error)) => {
                if let Some(duration) = duration_from_io_error(&method, &error, &mut backoff) {
                    tokio::time::sleep(duration).await;
                    continue;
                }

                return Err(Error::Io(error));
            }
            Err(error) => return Err(error),
        }
    }
}
