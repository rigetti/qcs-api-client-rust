use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::{Context, Poll},
};

use http::StatusCode;
use tonic::{
    body::BoxBody,
    client::GrpcService,
    codegen::http::{Request, Response},
};
use tower::Layer;

use qcs_api_client_common::configuration::{
    secrets::SecretAccessToken, tokens::TokenRefresher, ClientConfiguration, TokenError,
};

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
    #[allow(clippy::result_large_err)]
    pub fn new() -> Result<Self, Error<TokenError>> {
        let config = ClientConfiguration::load_default()?;
        Ok(Self::with_config(config))
    }

    /// Create a new [`RefreshLayer`] using the given QCS configuration profile.
    ///
    /// # Errors
    ///
    /// Will fail if loading the [`ClientConfiguration`] fails.
    #[allow(clippy::result_large_err)]
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
        super::common::pin_future_with_otel_context_if_available(service_call(
            req,
            token_refresher,
            service,
        ))
    }
}

async fn service_call<C, T>(
    req: Request<BoxBody>,
    token_refresher: T,
    mut channel: C,
) -> Result<Response<<C as GrpcService<BoxBody>>::ResponseBody>, Error<T::Error>>
where
    C: GrpcService<BoxBody> + Send,
    <C as GrpcService<BoxBody>>::ResponseBody: Send,
    <C as GrpcService<BoxBody>>::Future: Send,
    T: TokenRefresher + Send,
    T::Error: std::error::Error,
    Error<T::Error>: From<C::Error>,
{
    let token = token_refresher
        .validated_access_token()
        .await
        .map_err(Error::Refresh)?;
    let (req, retry_req) = super::build_duplicate_request(req).await?;
    let resp = make_request(&mut channel, req, token).await?;

    let grpc_authnz_failure = matches!(
        super::common::get_status_code_from_headers(resp.headers()).ok(),
        Some(tonic::Code::Unauthenticated) | Some(tonic::Code::PermissionDenied)
    );
    let http_authnz_failure =
        resp.status() == StatusCode::UNAUTHORIZED || resp.status() == StatusCode::FORBIDDEN;

    if grpc_authnz_failure || http_authnz_failure {
        #[cfg(feature = "tracing")]
        {
            tracing::info!("refreshing token after receiving unauthorized or forbidden status",);
        }

        // Refresh token and try again
        let token = token_refresher
            .validated_access_token()
            .await
            .map_err(Error::Refresh)?;

        #[cfg(feature = "tracing")]
        {
            tracing::info!("token refreshed");
        }
        // Ensure that the service is ready before trying to use it.
        // Failure to do this *will* cause a panic.
        poll_fn(|cx| channel.poll_ready(cx))
            .await
            .map_err(super::error::Error::from)?;
        make_request(&mut channel, retry_req, token).await
    } else {
        Ok(resp)
    }
}

async fn make_request<C, E: std::error::Error>(
    service: &mut C,
    mut request: Request<BoxBody>,
    access_token: SecretAccessToken,
) -> Result<Response<<C as GrpcService<BoxBody>>::ResponseBody>, Error<E>>
where
    C: GrpcService<BoxBody> + Send,
    <C as GrpcService<BoxBody>>::ResponseBody: Send,
    <C as GrpcService<BoxBody>>::Future: Send,
    Error<E>: From<C::Error>,
{
    let header_val = format!("Bearer {}", access_token.secret())
        .try_into()
        .map_err(Error::InvalidAccessToken)?;
    request.headers_mut().insert("authorization", header_val);
    service.call(request).await.map_err(Error::from)
}
