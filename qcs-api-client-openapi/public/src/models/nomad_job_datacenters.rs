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

/// NomadJobDatacenters : An enumeration.
use serde::{Deserialize, Serialize};

/// An enumeration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NomadJobDatacenters {
    #[serde(rename = "berkeley-775")]
    Berkeley775,
    #[serde(rename = "fremont-fab")]
    FremontFab,
    #[serde(rename = "rigetti-gb-1")]
    RigettiGb1,

    #[serde(untagged)]
    Unknown(String),
}

impl std::fmt::Display for NomadJobDatacenters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Berkeley775 => write!(f, "berkeley-775"),
            Self::FremontFab => write!(f, "fremont-fab"),
            Self::RigettiGb1 => write!(f, "rigetti-gb-1"),
            Self::Unknown(s) => s.fmt(f),
        }
    }
}

impl Default for NomadJobDatacenters {
    fn default() -> NomadJobDatacenters {
        Self::Berkeley775
    }
}
