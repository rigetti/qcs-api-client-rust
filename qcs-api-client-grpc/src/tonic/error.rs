use http::{header::InvalidHeaderValue, uri::InvalidUri};
use tonic::transport::Error as TransportError;

use qcs_api_client_common::configuration::LoadError;

use super::channel::ChannelError;

/// Errors that may occur when using gRPC.
#[derive(Debug, thiserror::Error)]
#[allow(variant_size_differences)]
pub enum Error<E>
where
    E: std::error::Error,
{
    /// Failed to refresh the access token.
    #[error("failed to refresh access token: {0}")]
    Refresh(#[source] E),
    /// Failed to load the QCS configuration.
    #[error("failed to load QCS config: {0}")]
    Load(#[from] LoadError),
    /// Failed to parse URI.
    #[error("failed to parse URI: {0}")]
    InvalidUri(#[from] InvalidUri),
    /// The gRPC call failed for some reason.
    #[error("service call failed with error: {0}")]
    Transport(#[from] TransportError),
    /// The provided access token is not a valid header value.
    #[error("access token is not a valid header value: {0}")]
    InvalidAccessToken(#[source] InvalidHeaderValue),
    /// The proxy configuration caused an error
    #[error("The channel configuration caused an error: {0}")]
    ChannelError(#[from] ChannelError),
    #[cfg(feature = "grpc-web")]
    #[error("The hyper grpc-web client returned an error: {0}")]
    HyperError(#[from] hyper::Error),
}
