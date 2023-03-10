pub mod channel {
    //! This module contains helper code for wrapping a [`Channel`] so that QCS credentials are
    //! automatically used and refreshed as necessary.
    //!
    //! Most users will want to use [`get_channel`], [`get_wrapped_channel`], or [`wrap_channel`].
    include!("../../qcs-api-client-grpc/src/channel.rs");
}
pub use channel::{get_channel, wrap_channel, wrap_channel_with_token_refresher};
pub use qcs_api_client_common::configuration as client_configuration;

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod google {
    pub mod protobuf {
        pub use pbjson_types::*;
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
