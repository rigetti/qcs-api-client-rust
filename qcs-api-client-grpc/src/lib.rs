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

pub use common::grpc::{get_channel, wrap_channel};
pub use qcs_api_client_common as common;

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod models {
    pub mod controller {
        tonic::include_proto!("models.controller");
        tonic::include_proto!("models.controller.serde");
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod services {
    pub mod controller {
        tonic::include_proto!("services.controller");
        tonic::include_proto!("services.controller.serde");
    }
    pub mod translation {
        tonic::include_proto!("services.translation");
        tonic::include_proto!("services.translation.serde");
    }
}
