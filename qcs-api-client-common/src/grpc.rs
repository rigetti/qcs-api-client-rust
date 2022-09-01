//! gRPC Common Code
//!

use crate::configuration::{ClientConfiguration, LoadError, RefreshError};
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::client::GrpcService;
use tonic::codegen::http::{Request, Response};
use tonic::transport::{Channel, Uri};
use tonic::{Code, Status};
use tower::{Layer, ServiceBuilder};

pub use tonic;
pub use tower;

/// Errors that may occur when using gRPC.
#[derive(Debug, thiserror::Error)]
#[allow(variant_size_differences)]
pub enum Error {
    /// The gRPC call failed for some reason.
    #[error("service call failed with error: {0}")]
    Status(#[from] Status),
    /// Failed to refresh the access token.
    #[error("failed to refresh access token: {0}")]
    Refresh(#[from] RefreshError),
    /// Failed to load the QCS configuration.
    #[error("failed to load QCS config: {0}")]
    Load(#[from] LoadError),
}

/// Get a [`Channel`] to the given [`Uri`] with QCS authentication set up already.
///
/// # Errors
///
/// See [`Error`]
pub async fn get_channel(uri: Uri) -> Result<RefreshService<Channel>, Error> {
    let inner = Channel::builder(uri)
        .user_agent(concat!(
            "QCS gRPC Client (Rust)/",
            env!("CARGO_PKG_VERSION")
        ))
        .expect("user agent string should be valid")
        .connect_lazy();

    wrap_channel(inner).await
}

/// Set up the given [`Channel`] with QCS authentication.
///
/// # Errors
///
/// See [`Error`]
pub async fn wrap_channel(channel: Channel) -> Result<RefreshService<Channel>, Error> {
    Ok(ServiceBuilder::new()
        .layer(RefreshLayer::new().await?)
        .service(channel))
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
    /// Will fail with error if loading the [`Configuration`] fails.
    pub async fn new() -> Result<Self, Error> {
        let config = ClientConfiguration::load().await?;
        Ok(Self::with_config(config))
    }

    /// Create a [`RefreshLayer`] from an existing [`Configuration`].
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

fn clone_request<T: Clone>(req: &Request<T>) -> Request<T> {
    let mut builder = Request::builder()
        .method(req.method().clone())
        .uri(req.uri().clone())
        .version(req.version());

    for (key, val) in req.headers().iter() {
        builder = builder.header(key.clone(), val.clone());
    }

    let new_req = builder
        .body(req.body().clone())
        .expect("all values from existing request should be valid");

    new_req
}

async fn make_request<S, T>(
    service: &mut S,
    mut request: Request<T>,
    token: String,
) -> Result<Response<S::ResponseBody>, Status>
where
    S: GrpcService<T, Error = Status> + Send,
    S::Future: Send,
    T: fmt::Debug + Send,
{
    let header_val = format!("Bearer {token}")
        .try_into()
        .expect("authorization header value should never be invalid");
    request.headers_mut().insert("authorization", header_val);
    service.call(request).await
}

impl<S, T> GrpcService<T> for RefreshService<S>
where
    S: GrpcService<T, Error = Status> + Clone + Send + 'static,
    S::Future: Send,
    T: fmt::Debug + Clone + Send + 'static,
{
    type ResponseBody = S::ResponseBody;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::from)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        let mut service = self.service.clone();
        let config = self.config.clone();
        Box::pin(async move {
            let token = config.get_bearer_access_token().await?;
            let retry_req = clone_request(&req);
            match make_request(&mut service, req, token).await {
                Ok(resp) => Ok(resp),
                Err(err) => match err.code() {
                    Code::PermissionDenied | Code::Unauthenticated => {
                        // Refresh token and try again
                        let token = config.refresh().await?;
                        make_request(&mut service, retry_req, token)
                            .await
                            .map_err(Error::from)
                    }
                    _ => Err(Error::Status(err)),
                },
            }
        })
    }
}
