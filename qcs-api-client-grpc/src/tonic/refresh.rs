use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use tonic::{
    body::BoxBody,
    client::GrpcService,
    codegen::http::{Request, Response},
};
use tower::Layer;

use qcs_api_client_common::configuration::{ClientConfiguration, TokenError, TokenRefresher};

#[cfg(feature = "tracing-opentelemetry")]
use super::traced_service_call;

#[cfg(not(feature = "tracing-opentelemetry"))]
use super::service_call;

use super::error::Error;

/// The [`GrpcService`] that wraps the gRPC client in order to provide QCS authentication.
///
/// See also: [`RefreshLayer`].
#[derive(Clone, Debug)]
pub struct RefreshService<S: GrpcService<BoxBody>, T: TokenRefresher> {
    service: S,
    token_refresher: T,
}

/// The [`Layer`] used to apply QCS authentication to requests.
#[derive(Clone, Debug)]
pub struct RefreshLayer<T: TokenRefresher> {
    token_refresher: T,
}

impl<T: TokenRefresher> RefreshLayer<T> {
    /// Create a new [`RefreshLayer`] with the given [`TokenRefresher`]
    pub const fn with_refresher(token_refresher: T) -> Self {
        Self { token_refresher }
    }
}

impl RefreshLayer<ClientConfiguration> {
    /// Create a new [`RefreshLayer`].
    ///
    /// # Errors
    ///
    /// Will fail with error if loading the [`ClientConfiguration`] fails.
    pub fn new() -> Result<Self, Error<TokenError>> {
        let config = ClientConfiguration::load_default()?;
        Ok(Self::with_config(config))
    }

    /// Create a new [`RefreshLayer`] using the given QCS configuration profile.
    ///
    /// # Errors
    ///
    /// Will fail if loading the [`ClientConfiguration`] fails.
    pub fn with_profile(profile: String) -> Result<Self, Error<TokenError>> {
        let config = ClientConfiguration::load_profile(profile)?;
        Ok(Self::with_config(config))
    }

    /// Create a [`RefreshLayer`] from an existing [`ClientConfiguration`].
    #[must_use]
    pub const fn with_config(config: ClientConfiguration) -> Self {
        Self::with_refresher(config)
    }
}

impl<S, T> Layer<S> for RefreshLayer<T>
where
    S: GrpcService<BoxBody>,
    T: TokenRefresher + Clone,
{
    type Service = RefreshService<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        RefreshService {
            token_refresher: self.token_refresher.clone(),
            service: inner,
        }
    }
}

impl<S, T> GrpcService<BoxBody> for RefreshService<S, T>
where
    S: GrpcService<BoxBody> + Clone + Send + 'static,
    <S as GrpcService<BoxBody>>::Future: Send,
    <S as GrpcService<BoxBody>>::ResponseBody: Send,
    T: TokenRefresher + Clone + Send + 'static,
    T::Error: std::error::Error + Sync,
    Error<T::Error>: From<S::Error>,
    <T as TokenRefresher>::Error: Send,
{
    type ResponseBody = <S as GrpcService<BoxBody>>::ResponseBody;
    type Error = Error<T::Error>;
    type Future =
        Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Error::from)
    }

    fn call(&mut self, req: Request<BoxBody>) -> Self::Future {
        let service = self.service.clone();
        // It is necessary to replace self.service with the above clone
        // because the cloned version may not be "ready".
        //
        // See this github issue for more context:
        // https://github.com/tower-rs/tower/issues/547
        let service = std::mem::replace(&mut self.service, service);
        let token_refresher = self.token_refresher.clone();
        Box::pin(async move {
            #[cfg(feature = "tracing-opentelemetry")]
            use opentelemetry_api::trace::FutureExt;

            #[cfg(feature = "tracing-opentelemetry")]
            return traced_service_call(req, token_refresher, service)
                .with_current_context()
                .await;

            #[cfg(not(feature = "tracing-opentelemetry"))]
            return service_call(req, token_refresher, service).await;
        })
    }
}
