pub use common::grpc::{get_channel, wrap_channel};
pub use qcs_api_client_common as common;

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod google {
    pub mod protobuf {
        tonic::include_proto!("google.protobuf");
    }
}
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
