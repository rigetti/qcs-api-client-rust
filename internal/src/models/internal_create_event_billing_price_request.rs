/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://gitlab.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://gitlab.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

/// InternalCreateEventBillingPriceRequest : Create an event price record. This maps billing account, event, and quantum processor identifier to a Stripe [price](https://stripe.com/docs/api/prices/object) identifier. Consumers of the `EventBillingPrice` will use the Stripe price identifer to create a Stripe [invoice item](https://stripe.com/docs/api/invoiceitems) or [subscription item](https://stripe.com/docs/api/subscription_items). The content of the Stripe price object is opaque to QCS services by design. Note, that the request must set `quantumProcessorId` if it sets account parameters.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InternalCreateEventBillingPriceRequest {
    /// userId for `accountType` \"user\", group name for `accountType` \"group\".
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<crate::models::AccountType>,
    #[serde(rename = "billingPrice")]
    pub billing_price: Box<crate::models::BillingPrice>,
    #[serde(rename = "product")]
    pub product: Product,
    #[serde(rename = "quantumProcessorId", skip_serializing_if = "Option::is_none")]
    pub quantum_processor_id: Option<String>,
}

impl InternalCreateEventBillingPriceRequest {
    /// Create an event price record. This maps billing account, event, and quantum processor identifier to a Stripe [price](https://stripe.com/docs/api/prices/object) identifier. Consumers of the `EventBillingPrice` will use the Stripe price identifer to create a Stripe [invoice item](https://stripe.com/docs/api/invoiceitems) or [subscription item](https://stripe.com/docs/api/subscription_items). The content of the Stripe price object is opaque to QCS services by design. Note, that the request must set `quantumProcessorId` if it sets account parameters.
    pub fn new(
        billing_price: crate::models::BillingPrice,
        product: Product,
    ) -> InternalCreateEventBillingPriceRequest {
        InternalCreateEventBillingPriceRequest {
            account_id: None,
            account_type: None,
            billing_price: Box::new(billing_price),
            product,
            quantum_processor_id: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Product {
    #[serde(rename = "qpuJobCompletion")]
    QpuJobCompletion,
    #[serde(rename = "qpuJobTime")]
    QpuJobTime,
    #[serde(rename = "reservationCreation")]
    ReservationCreation,
}

impl Default for Product {
    fn default() -> Product {
        Self::QpuJobCompletion
    }
}
