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

/// BillingPrice : A configuration for calculating the cost of `BillingProduct` usage based on quantity, and when that cost should be added as an invoice item.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BillingPrice {
    /// Whether the price can be used for new purchases.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "billingScheme", skip_serializing_if = "Option::is_none")]
    pub billing_scheme: Option<crate::models::BillingPriceScheme>,
    /// Unique identifier for the object.
    #[serde(rename = "id")]
    pub id: String,
    /// This object's type, which is always `price`.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    /// Use `one_time` to invoice immediately based on a single usage report, e.g. purchasing a QPU reservation. Use `recurring` to aggregate usage reports over an interval and then invoice once based on `BillingPriceRecurrence`, e.g. on-demand QPU usage.
    #[serde(rename = "priceType", skip_serializing_if = "Option::is_none")]
    pub price_type: Option<PriceType>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<Box<crate::models::BillingProduct>>,
    #[serde(rename = "recurring", skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Box<crate::models::BillingPriceRecurrence>>,
    /// Configure how price should be calculated based on quantity when `billingScheme=tiered`. Requires at least two tiers.
    #[serde(rename = "tiers", skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<crate::models::BillingPriceTier>>,
    #[serde(rename = "tiersMode", skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<crate::models::BillingPriceTiersMode>,
    /// The amount of `currency` to charge per quantity used. Requires that `billingScheme=per_unit`.
    #[serde(rename = "unitAmountDecimal", skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<f64>,
}

impl BillingPrice {
    /// A configuration for calculating the cost of `BillingProduct` usage based on quantity, and when that cost should be added as an invoice item.
    pub fn new(id: String) -> BillingPrice {
        BillingPrice {
            active: None,
            billing_scheme: None,
            id,
            object: None,
            price_type: None,
            product: None,
            recurring: None,
            tiers: None,
            tiers_mode: None,
            unit_amount_decimal: None,
        }
    }
}

/// This object's type, which is always `price`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "price")]
    Price,
}

impl Default for Object {
    fn default() -> Object {
        Self::Price
    }
}
/// Use `one_time` to invoice immediately based on a single usage report, e.g. purchasing a QPU reservation. Use `recurring` to aggregate usage reports over an interval and then invoice once based on `BillingPriceRecurrence`, e.g. on-demand QPU usage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PriceType {
    #[serde(rename = "one_time")]
    OneTime,
    #[serde(rename = "recurring")]
    Recurring,
}

impl Default for PriceType {
    fn default() -> PriceType {
        Self::OneTime
    }
}
