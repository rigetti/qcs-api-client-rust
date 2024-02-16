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

/// struct for typed errors of method [`get_health`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHealthError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`health_check`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HealthCheckError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`health_check_deprecated`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HealthCheckDeprecatedError {
    Status422(crate::models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_create_product_billing_price`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalCreateProductBillingPriceError {
    Status403(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_list_product_billing_prices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalListProductBillingPricesError {
    Status403(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_update_group_billing_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalUpdateGroupBillingCustomerError {
    Status403(crate::models::Error),
    Status404(crate::models::Error),
    Status409(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_update_product_billing_price`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalUpdateProductBillingPriceError {
    Status403(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`internal_update_user_billing_customer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalUpdateUserBillingCustomerError {
    Status403(crate::models::Error),
    Status404(crate::models::Error),
    Status409(crate::models::Error),
    Status422(crate::models::Error),
    UnknownValue(serde_json::Value),
}

async fn get_health_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
) -> Result<crate::models::Health, Error<GetHealthError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/", local_var_configuration.qcs_config.api_url());
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
                "making get_health request",
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
        let local_var_entity: Option<GetHealthError> =
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

/// Retrieve the health status of the API
pub async fn get_health(
    configuration: &configuration::Configuration,
) -> Result<crate::models::Health, Error<GetHealthError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = get_health_inner(configuration, &mut backoff).await;

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
async fn health_check_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
) -> Result<serde_json::Value, Error<HealthCheckError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/healthcheck",
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
                "making health_check request",
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
        let local_var_entity: Option<HealthCheckError> =
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

/// Endpoint to return a status 200 for load balancer health checks
pub async fn health_check(
    configuration: &configuration::Configuration,
) -> Result<serde_json::Value, Error<HealthCheckError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = health_check_inner(configuration, &mut backoff).await;

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
async fn health_check_deprecated_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
) -> Result<serde_json::Value, Error<HealthCheckDeprecatedError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/", local_var_configuration.qcs_config.api_url());
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
                "making health_check_deprecated request",
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
        let local_var_entity: Option<HealthCheckDeprecatedError> =
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

/// Endpoint to return a status 200 for load balancer health checks
pub async fn health_check_deprecated(
    configuration: &configuration::Configuration,
) -> Result<serde_json::Value, Error<HealthCheckDeprecatedError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = health_check_deprecated_inner(configuration, &mut backoff).await;

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
async fn internal_create_product_billing_price_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    product: &str,
    internal_create_product_billing_price_request: crate::models::InternalCreateProductBillingPriceRequest,
) -> Result<crate::models::BillingPrice, Error<InternalCreateProductBillingPriceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/products/{product}/billingPrices",
        local_var_configuration.qcs_config.api_url(),
        product = crate::apis::urlencode(product)
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
                "making internal_create_product_billing_price request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder =
        local_var_req_builder.json(&internal_create_product_billing_price_request);

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
        let local_var_entity: Option<InternalCreateProductBillingPriceError> =
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

pub async fn internal_create_product_billing_price(
    configuration: &configuration::Configuration,
    product: &str,
    internal_create_product_billing_price_request: crate::models::InternalCreateProductBillingPriceRequest,
) -> Result<crate::models::BillingPrice, Error<InternalCreateProductBillingPriceError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = internal_create_product_billing_price_inner(
            configuration,
            &mut backoff,
            product.clone(),
            internal_create_product_billing_price_request.clone(),
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
async fn internal_list_product_billing_prices_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    product: crate::models::Product,
    use_test_product: Option<bool>,
    filter: Option<&str>,
    page_size: Option<i64>,
    page_token: Option<&str>,
) -> Result<
    crate::models::InternalListProductBillingPricesResponse,
    Error<InternalListProductBillingPricesError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/products/{product}/billingPrices",
        local_var_configuration.qcs_config.api_url(),
        product = product
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
                "making internal_list_product_billing_prices request",
            );
        }
    }

    if let Some(ref local_var_str) = use_test_product {
        local_var_req_builder =
            local_var_req_builder.query(&[("useTestProduct", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageToken", &local_var_str.to_string())]);
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
        let local_var_entity: Option<InternalListProductBillingPricesError> =
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

/// List billing prices for the requested product.  Available filter fields include:  * `active` - boolean * `type` - string
pub async fn internal_list_product_billing_prices(
    configuration: &configuration::Configuration,
    product: crate::models::Product,
    use_test_product: Option<bool>,
    filter: Option<&str>,
    page_size: Option<i64>,
    page_token: Option<&str>,
) -> Result<
    crate::models::InternalListProductBillingPricesResponse,
    Error<InternalListProductBillingPricesError>,
> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = internal_list_product_billing_prices_inner(
            configuration,
            &mut backoff,
            product.clone(),
            use_test_product.clone(),
            filter.clone(),
            page_size.clone(),
            page_token.clone(),
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
async fn internal_update_group_billing_customer_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    group_name: &str,
    account_billing_customer_update_request: Option<
        crate::models::AccountBillingCustomerUpdateRequest,
    >,
) -> Result<crate::models::BillingCustomer, Error<InternalUpdateGroupBillingCustomerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/groups/{groupName}/billingCustomer",
        local_var_configuration.qcs_config.api_url(),
        groupName = crate::apis::urlencode(group_name)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
                method="PATCH",
                "making internal_update_group_billing_customer request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&account_billing_customer_update_request);

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
        let local_var_entity: Option<InternalUpdateGroupBillingCustomerError> =
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

/// Update the billing customer assigned to a QCS group account. This does not create the customer in Stripe and will validate that the billing customer exists and has not been assigned to any other QCS account. It will add the appropriate metadata to the existing billing customer.
pub async fn internal_update_group_billing_customer(
    configuration: &configuration::Configuration,
    group_name: &str,
    account_billing_customer_update_request: Option<
        crate::models::AccountBillingCustomerUpdateRequest,
    >,
) -> Result<crate::models::BillingCustomer, Error<InternalUpdateGroupBillingCustomerError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = internal_update_group_billing_customer_inner(
            configuration,
            &mut backoff,
            group_name.clone(),
            account_billing_customer_update_request.clone(),
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
async fn internal_update_product_billing_price_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    billing_price_id: &str,
    internal_update_product_billing_price_request: crate::models::InternalUpdateProductBillingPriceRequest,
) -> Result<crate::models::BillingPrice, Error<InternalUpdateProductBillingPriceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/billingPrices/{billingPriceId}",
        local_var_configuration.qcs_config.api_url(),
        billingPriceId = crate::apis::urlencode(billing_price_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
                method="PATCH",
                "making internal_update_product_billing_price request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder =
        local_var_req_builder.json(&internal_update_product_billing_price_request);

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
        let local_var_entity: Option<InternalUpdateProductBillingPriceError> =
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

pub async fn internal_update_product_billing_price(
    configuration: &configuration::Configuration,
    billing_price_id: &str,
    internal_update_product_billing_price_request: crate::models::InternalUpdateProductBillingPriceRequest,
) -> Result<crate::models::BillingPrice, Error<InternalUpdateProductBillingPriceError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = internal_update_product_billing_price_inner(
            configuration,
            &mut backoff,
            billing_price_id.clone(),
            internal_update_product_billing_price_request.clone(),
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
async fn internal_update_user_billing_customer_inner(
    configuration: &configuration::Configuration,
    backoff: &mut ExponentialBackoff,
    user_id: &str,
    account_billing_customer_update_request: Option<
        crate::models::AccountBillingCustomerUpdateRequest,
    >,
) -> Result<crate::models::BillingCustomer, Error<InternalUpdateUserBillingCustomerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/internal/users/{userId}/billingCustomer",
        local_var_configuration.qcs_config.api_url(),
        userId = crate::apis::urlencode(user_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
                method="PATCH",
                "making internal_update_user_billing_customer request",
            );
        }
    }

    // Use QCS Bearer token
    let token = configuration.qcs_config.get_bearer_access_token().await?;
    local_var_req_builder = local_var_req_builder.bearer_auth(token);

    local_var_req_builder = local_var_req_builder.json(&account_billing_customer_update_request);

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
        let local_var_entity: Option<InternalUpdateUserBillingCustomerError> =
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

/// Update the billing customer assigned to a QCS user account. This does not create the customer in Stripe and will validate that the billing customer exists and has not been assigned to any other QCS account.
pub async fn internal_update_user_billing_customer(
    configuration: &configuration::Configuration,
    user_id: &str,
    account_billing_customer_update_request: Option<
        crate::models::AccountBillingCustomerUpdateRequest,
    >,
) -> Result<crate::models::BillingCustomer, Error<InternalUpdateUserBillingCustomerError>> {
    let mut backoff = configuration.backoff.clone();
    let mut refreshed_credentials = false;
    loop {
        let result = internal_update_user_billing_customer_inner(
            configuration,
            &mut backoff,
            user_id.clone(),
            account_billing_customer_update_request.clone(),
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
