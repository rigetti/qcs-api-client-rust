/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://github.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://github.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// InternalCreateProductBillingPriceRequest : Defines a new price to be created in our billing vendor. Note, creating `BillingPrice` does _not_ assign the resulting `BillingPrice` to an `EventBillingPrice`.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InternalCreateProductBillingPriceRequest {
    #[serde(rename = "billingScheme", skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<crate::models::BillingPriceScheme>,
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Box<crate::models::BillingPriceRecurrence>>,
    /// Each element represents a pricing tier. This parameter requires `billingScheme` to be set to `tiered`. See also the documentation for `billingScheme`.
    #[serde(rename = "tiers", skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<crate::models::BillingPriceTier>>,
    #[serde(rename = "tiersMode", skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<crate::models::BillingPriceTiersMode>,
    /// The unit amount in `currency` to be charged. Only set if `billingScheme=per_unit`.
    #[serde(rename = "unitAmountDecimal", skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<f64>,
    /// If set to true, use a product that is explicitly marked as a product for testing. This is helpful for isolating test data within the production environment.
    #[serde(rename = "useTestProduct")]
    pub use_test_product: bool,
}

impl InternalCreateProductBillingPriceRequest {
    /// Defines a new price to be created in our billing vendor. Note, creating `BillingPrice` does _not_ assign the resulting `BillingPrice` to an `EventBillingPrice`.
    pub fn new(use_test_product: bool) -> InternalCreateProductBillingPriceRequest {
        InternalCreateProductBillingPriceRequest {
            billing_scheme: None,
            recurring: None,
            tiers: None,
            tiers_mode: None,
            unit_amount_decimal: None,
            use_test_product,
        }
    }
}
