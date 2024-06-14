pub mod tonic {
    //! Utilities for creating and working with [`Channel`]s.
    //!
    //! This module contains helper code for wrapping a [`Channel`] so that QCS credentials are
    //! automatically used and refreshed as necessary.
    //!
    //! Most users will want to use [`get_channel`], [`get_wrapped_channel`], or [`wrap_channel`].
    //!
    //! # Generics
    //!
    //! The functions for wrapping channels use generics `C: GrpcService<BoxBody>` to accept not only
    //! bare [`Channel`]s but also channels wrapped in middleware like [`RefreshService`] or
    //! [`RetryService`]
    //!
    //! # Example
    //!
    //! To create a channel that automatically retries on certain errors and refreshes QCS credentials
    //! when authentication fails:
    //!
    //! ```no_run
    //! # #[tokio::main]
    //! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //! use qcs_api_client_grpc_internal::tonic::parse_uri;
    //! use qcs_api_client_grpc_internal::tonic::get_channel;
    //! use qcs_api_client_grpc_internal::tonic::wrap_channel;
    //! use qcs_api_client_grpc_internal::tonic::wrap_channel_with_retry;
    //!
    //! let uri = parse_uri("https://api.qcs.rigetti.com")?;
    //! let channel = get_channel(uri)?;
    //! let with_creds = wrap_channel(channel)?;
    //! let with_creds_and_retry = wrap_channel_with_retry(with_creds);
    //! // Use with_creds_and_retry as a gRPC client
    //! # Ok(())
    //! # }
    //! ```
    include!("../../qcs-api-client-grpc/src/tonic/mod.rs");
}
pub use qcs_api_client_common::configuration as client_configuration;
pub use tonic::{get_channel, wrap_channel, wrap_channel_with_token_refresher};

#[allow(clippy::derive_partial_eq_without_eq)]
pub mod google {
    pub mod protobuf {
        pub use pbjson_types::*;
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::needless_borrow)]
pub mod models {
    pub mod controller {
        tonic::include_proto!("models.controller");
        tonic::include_proto!("models.controller.serde");
    }
    pub mod translation {
        tonic::include_proto!("models.translation");
        tonic::include_proto!("models.translation.serde");
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(clippy::large_enum_variant)]
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
