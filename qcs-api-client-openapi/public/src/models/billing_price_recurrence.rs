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

/// BillingPriceRecurrence : How to invoice for the usage of a product that has a recurring (subscription) price.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BillingPriceRecurrence {
    /// How to determine the aggregate usage over the `interval` when `usageType=metered`. Using `sum` is recommended.
    #[serde(rename = "aggregateUsage", skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<AggregateUsage>,
    /// The frequency at which recurring usage should be billed. Using `month` is recommended.
    #[serde(rename = "interval")]
    pub interval: Interval,
    /// The number of `interval` units between each billing cycle. For example, `interval=month` and `intervalCount=1` means every month (recommended).
    #[serde(rename = "intervalCount", skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i64>,
    /// Use `metered` to calculate a dynamic quantity based on reported usage records (recommended). Use `licensed` when you provide a fixed quantity, e.g. a TV subscription.
    #[serde(rename = "usageType", skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
}

impl BillingPriceRecurrence {
    /// How to invoice for the usage of a product that has a recurring (subscription) price.
    pub fn new(interval: Interval) -> BillingPriceRecurrence {
        BillingPriceRecurrence {
            aggregate_usage: None,
            interval,
            interval_count: None,
            usage_type: None,
        }
    }
}

/// How to determine the aggregate usage over the `interval` when `usageType=metered`. Using `sum` is recommended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregateUsage {
    #[serde(rename = "last_during_period")]
    LastDuringPeriod,
    #[serde(rename = "last_ever")]
    LastEver,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "sum")]
    Sum,
}

impl Default for AggregateUsage {
    fn default() -> AggregateUsage {
        Self::LastDuringPeriod
    }
}
/// The frequency at which recurring usage should be billed. Using `month` is recommended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Interval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

impl Default for Interval {
    fn default() -> Interval {
        Self::Day
    }
}
/// Use `metered` to calculate a dynamic quantity based on reported usage records (recommended). Use `licensed` when you provide a fixed quantity, e.g. a TV subscription.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageType {
    #[serde(rename = "licensed")]
    Licensed,
    #[serde(rename = "metered")]
    Metered,
}

impl Default for UsageType {
    fn default() -> UsageType {
        Self::Licensed
    }
}
