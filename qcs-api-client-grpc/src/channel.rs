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

use crate::client_configuration::{ClientConfiguration, LoadError, RefreshError};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use http_body::Full;
use tonic::client::GrpcService;
use tonic::codegen::http::{Request, Response, StatusCode};
use tonic::transport::{Channel, Uri, Error as TransportError};
use tonic::Status;
use tower::{Layer, ServiceBuilder};
use tonic::body::BoxBody;
use tonic::codegen::Body;
use tonic::codegen::http::uri::InvalidUri;

/// Errors that may occur when using gRPC.
#[derive(Debug, thiserror::Error)]
#[allow(variant_size_differences)]
pub enum Error {
    /// Failed to refresh the access token.
    #[error("failed to refresh access token: {0}")]
    Refresh(#[from] RefreshError),
    /// Failed to load the QCS configuration.
    #[error("failed to load QCS config: {0}")]
    Load(#[from] LoadError),
    /// Failed to parse URI.
    #[error("failed to parse URI: {0}")]
    InvalidUri(#[from] InvalidUri),
    /// The gRPC call failed for some reason.
    #[error("service call failed with error: {0}")]
    Transport(#[from] TransportError),
}

/// Parse a string as a URI.
///
/// This serves as a helper to avoid consumers needing to create a new error just to include this.
///
/// # Errors
///
/// [`Error::InvalidUri`] if the string is an invalid URI.
pub fn parse_uri(s: &str) -> Result<Uri, Error> {
    s.parse().map_err(Error::from)
}

/// Get a [`Channel`] to the given [`Uri`].
///
/// Sets up things like user agent without setting up QCS credentials.
pub fn get_channel(uri: Uri) -> Channel {
    Channel::builder(uri)
        .user_agent(concat!(
        "QCS gRPC Client (Rust)/",
        env!("CARGO_PKG_VERSION")
        ))
        .expect("user agent string should be valid")
        .connect_lazy()
}

/// Get a [`Channel`] to the given [`Uri`] with QCS authentication set up already.
///
/// # Errors
///
/// See [`Error`]
pub async fn get_wrapped_channel(uri: Uri) -> Result<RefreshService<Channel>, Error> {
    wrap_channel(get_channel(uri)).await
}

/// Set up the given [`Channel`] with QCS authentication.
#[must_use]
pub fn wrap_channel_with(channel: Channel, config: ClientConfiguration) -> RefreshService<Channel> {
    ServiceBuilder::new()
        .layer(RefreshLayer::with_config(config))
        .service(channel)
}

/// Set up the given [`Channel`] with QCS authentication.
///
/// # Errors
///
/// See [`Error`]
pub async fn wrap_channel(channel: Channel) -> Result<RefreshService<Channel>, Error> {
    Ok(wrap_channel_with(channel, ClientConfiguration::load().await?))
}

/// The [`Layer`] used to apply QCS authentication to all gRPC calls.
#[derive(Clone, Debug)]
pub struct RefreshLayer {
    config: ClientConfiguration,
}

impl RefreshLayer {
    /// Create a new [`RefreshLayer`].
    ///
    /// # Errors
    ///
    /// Will fail with error if loading the [`ClientConfiguration`] fails.
    pub async fn new() -> Result<Self, Error> {
        let config = ClientConfiguration::load().await?;
        Ok(Self::with_config(config))
    }

    /// Create a [`RefreshLayer`] from an existing [`ClientConfiguration`].
    #[must_use]
    pub const fn with_config(config: ClientConfiguration) -> Self {
        Self { config }
    }
}

impl<S> Layer<S> for RefreshLayer {
    type Service = RefreshService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RefreshService {
            config: self.config.clone(),
            service: inner,
        }
    }
}

/// The [`GrpcService`] that wraps the gRPC client in order to provide QCS authentication.
///
/// See also: [`RefreshLayer`].
#[derive(Clone, Debug)]
pub struct RefreshService<S> {
    service: S,
    config: ClientConfiguration,
}

type CloneBodyError = <BoxBody as Body>::Error;

async fn clone_body(body: Request<BoxBody>) -> Result<(BoxBody, BoxBody), CloneBodyError> {
    let mut bytes = Vec::new();
    let mut body = body.into_body();
    while let Some(result) = body.data().await {
        bytes.extend(result.expect("loading request body should not fail here"));
    }
    let bytes = Full::from(bytes)
        .map_err(|_| Status::internal("this will never happen from Infallible"));
    Ok((BoxBody::new(bytes.clone()), BoxBody::new(bytes)))
}

async fn clone_request(req: Request<BoxBody>) -> (Request<BoxBody>, Request<BoxBody>) {
    let mut builder_1 = Request::builder()
        .method(req.method().clone())
        .uri(req.uri().clone())
        .version(req.version());

    let mut builder_2 = Request::builder()
        .method(req.method().clone())
        .uri(req.uri().clone())
        .version(req.version());

    for (key, val) in req.headers().iter() {
        builder_1 = builder_1.header(key.clone(), val.clone());
        builder_2 = builder_2.header(key.clone(), val.clone());
    }

    let (body_1, body_2) = clone_body(req).await.unwrap();

    let req_1 = builder_1
        .body(body_1)
        .expect("all values from existing request should be valid");

    let req_2 = builder_2
        .body(body_2)
        .expect("all values from existing request should be valid");

    (req_1, req_2)
}

async fn make_request(
    service: &mut Channel,
    mut request: Request<BoxBody>,
    token: String,
) -> Result<Response<<Channel as GrpcService<BoxBody>>::ResponseBody>, TransportError> {
    let header_val = format!("Bearer {token}")
        .try_into()
        .expect("authorization header value should never be invalid");
    request.headers_mut().insert("authorization", header_val);
    service.call(request).await
}

impl<> GrpcService<BoxBody> for RefreshService<Channel>
    where
        Channel: GrpcService<BoxBody, Error=TransportError> + Clone + Send + 'static,
        <Channel as GrpcService<BoxBody>>::Future: Send,
{
    type ResponseBody = <Channel as GrpcService<BoxBody>>::ResponseBody;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Response<Self::ResponseBody>, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::from)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        let mut service = self.service.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let token = config.get_bearer_access_token().await?;
            let (req, retry_req) = clone_request(req).await;
            let resp = make_request(&mut service, req, token).await?;
            match resp.status() {
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                    // Refresh token and try again
                    let token = config.refresh().await?;
                    make_request(&mut service, retry_req, token)
                        .await
                        .map_err(Error::from)
                },
                _ => Ok(resp),
            }
        })
    }
}