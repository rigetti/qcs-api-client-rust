/// QCS Middleware for [`tonic`] clients.
use http::StatusCode;
use http_body::{Body, Full};
use tonic::{
    body::BoxBody,
    client::GrpcService,
    codegen::http::{Request, Response},
    Status,
};

#[cfg(feature = "tracing")]
use tonic::transport::Uri;

mod channel;
mod error;
#[cfg(feature = "grpc-web")]
mod grpc_web;
mod refresh;
mod retry;

pub use channel::*;
pub use error::*;
#[cfg(feature = "grpc-web")]
pub use grpc_web::*;
pub use refresh::*;
pub use retry::*;

use qcs_api_client_common::configuration::TokenRefresher;

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
    #[cfg(feature = "tracing")]
    {
        if should_trace(
            &token_refresher,
            &get_full_url_string(&token_refresher, req.uri()),
            true,
        ) {
            tracing::debug!("making gRPC request to {}", req.uri());
        }
    }

    let token = match token_refresher
        .get_access_token()
        .await
        .map_err(Error::Refresh)?
    {
        Some(token) => token,
        None => token_refresher
            .refresh_access_token()
            .await
            .map_err(Error::Refresh)?,
    };

    let (req, retry_req) = clone_request(req).await;
    let resp = make_request(&mut channel, req, token).await?;
    match resp.status() {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            // Refresh token and try again
            let token = token_refresher
                .refresh_access_token()
                .await
                .map_err(Error::Refresh)?;
            make_request(&mut channel, retry_req, token).await
        }
        _ => Ok(resp),
    }
}

async fn clone_body(body: Request<BoxBody>) -> (BoxBody, BoxBody) {
    let mut bytes = Vec::new();
    let mut body = body.into_body();
    while let Some(result) = body.data().await {
        bytes.extend(result.expect("loading request body should not fail here"));
    }

    let bytes =
        Full::from(bytes).map_err(|_| Status::internal("this will never happen from Infallible"));
    (BoxBody::new(bytes.clone()), BoxBody::new(bytes))
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

    for (key, val) in req.headers() {
        builder_1 = builder_1.header(key.clone(), val.clone());
        builder_2 = builder_2.header(key.clone(), val.clone());
    }

    let (body_1, body_2) = clone_body(req).await;

    let req_1 = builder_1
        .body(body_1)
        .expect("all values from existing request should be valid");

    let req_2 = builder_2
        .body(body_2)
        .expect("all values from existing request should be valid");

    (req_1, req_2)
}

async fn make_request<C, E: std::error::Error>(
    service: &mut C,
    mut request: Request<BoxBody>,
    token: String,
) -> Result<Response<<C as GrpcService<BoxBody>>::ResponseBody>, Error<E>>
where
    C: GrpcService<BoxBody> + Send,
    <C as GrpcService<BoxBody>>::ResponseBody: Send,
    <C as GrpcService<BoxBody>>::Future: Send,
    Error<E>: From<C::Error>,
{
    let header_val = format!("Bearer {token}")
        .try_into()
        .map_err(Error::InvalidAccessToken)?;
    request.headers_mut().insert("authorization", header_val);
    service.call(request).await.map_err(Error::from)
}

#[cfg(feature = "tracing")]
fn get_full_url_string<T: TokenRefresher>(token_refresher: &T, uri: &Uri) -> String {
    format!("{}{}", token_refresher.base_url(), uri)
}

#[cfg(feature = "tracing")]
fn should_trace<T: TokenRefresher>(token_refresher: &T, url_str: &str, default: bool) -> bool {
    use urlpattern::UrlPatternMatchInput;

    let url = url_str.parse::<::url::Url>().ok();

    url.map_or(default, |url| {
        token_refresher.should_trace(&UrlPatternMatchInput::Url(url))
    })
}

#[cfg(feature = "tracing-opentelemetry")]
async fn make_traced_request<C, E: std::error::Error>(
    service: &mut C,
    mut request: Request<BoxBody>,
    token: String,
) -> Result<Response<<C as GrpcService<BoxBody>>::ResponseBody>, Error<E>>
where
    C: GrpcService<BoxBody> + Send,
    <C as GrpcService<BoxBody>>::Future: Send,
    Error<E>: From<C::Error>,
{
    use opentelemetry::trace::FutureExt;
    use tracing::Instrument;

    let header_val = format!("Bearer {token}")
        .try_into()
        .map_err(Error::InvalidAccessToken)?;
    request.headers_mut().insert("authorization", header_val);

    let span = make_grpc_request_span(&request);
    service
        .call(request)
        .with_current_context()
        .instrument(span)
        .await
        .map_err(Error::from)
}

#[cfg(feature = "tracing-opentelemetry")]
async fn traced_service_call<C, T: TokenRefresher + Send>(
    original_req: Request<BoxBody>,
    config: T,
    mut channel: C,
) -> Result<Response<<C as GrpcService<BoxBody>>::ResponseBody>, Error<T::Error>>
where
    C: GrpcService<BoxBody> + Send,
    <C as GrpcService<BoxBody>>::ResponseBody: Send,
    <C as GrpcService<BoxBody>>::Future: Send,
    T::Error: std::error::Error,
    Error<T::Error>: From<C::Error>,
{
    use opentelemetry::{propagation::TextMapPropagator, sdk::propagation::TraceContextPropagator};
    use opentelemetry_api::trace::FutureExt;
    use opentelemetry_http::HeaderInjector;
    use qcs_api_client_common::tracing_configuration::TracingConfiguration;

    // The request URI here doesn't include the base url, so we have  to manually add it here to evaluate request filter patterns.
    let full_request_url = format!("{}{}", config.base_url(), &original_req.uri());

    if should_trace(&config, &full_request_url, true) {
        tracing::debug!("making traced gRPC request to {}", full_request_url);
    }

    let should_otel_trace =
        config.tracing_configuration().is_some() && should_trace(&config, &full_request_url, false);

    if !should_otel_trace {
        return service_call(original_req, config, channel).await;
    }

    // request an access token if one hasn't been requested yet
    let token = match config
        .get_access_token()
        .with_current_context()
        .await
        .map_err(Error::Refresh)?
    {
        Some(token) => token,
        None => config
            .refresh_access_token()
            .with_current_context()
            .await
            .map_err(Error::Refresh)?,
    };

    let (mut req, mut retry_req) = clone_request(original_req).with_current_context().await;

    if config
        .tracing_configuration()
        .is_some_and(TracingConfiguration::propagate_otel_context)
    {
        // Poor semantics here, but adding custom gRPC metadata is equivalent to setting request
        // headers: https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/doc/PROTOCOL-HTTP2.md.
        let propagator = TraceContextPropagator::new();
        let mut injector = HeaderInjector(req.headers_mut());
        propagator.inject_context(&opentelemetry::Context::current(), &mut injector);
    }

    let resp = make_traced_request(&mut channel, req, token)
        .with_current_context()
        .await?;

    match resp.status() {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            tracing::info!("refreshing token after receiving unauthorized or forbidden status",);
            // Refresh token and try again
            let token = config
                .refresh_access_token()
                .with_current_context()
                .await
                .map_err(Error::Refresh)?;
            tracing::info!("token refreshed");
            let propagator = TraceContextPropagator::new();
            let mut injector = HeaderInjector(retry_req.headers_mut());
            propagator.inject_context(&opentelemetry::Context::current(), &mut injector);

            make_traced_request(&mut channel, retry_req, token)
                .with_current_context()
                .await
        }
        _ => Ok(resp),
    }
}

#[cfg(feature = "tracing-opentelemetry")]
static GRPC_SPAN_NAME: &str = "gRPC request";

/// Creates a gRPC request span that conforms to the gRPC semantic conventions. See <https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/rpc.md> for details.
#[cfg(feature = "tracing-opentelemetry")]
fn make_grpc_request_span(request: &Request<BoxBody>) -> tracing::Span {
    let _method = request.method();
    let url = request.uri();
    let path = url.path();
    let mut path_split = path.split('/');
    let (_, service, method) = (path_split.next(), path_split.next(), path_split.next());
    let service = service.unwrap_or("");
    let method = method.unwrap_or("");
    let _scheme = url.scheme();
    let host = url.host().unwrap_or("");
    let host_port = url.port().map_or(0u16, |p| p.as_u16());
    tracing::span!(
        tracing::Level::INFO,
        GRPC_SPAN_NAME,
        rpc.system = "grpc",
        rpc.service = %service,
        rpc.method = %method,
        net.peer.name = %host,
        net.peer.port = %host_port,
        "message.type" = "sent",
        // We would like to include this attribute according to the gRPC semantic conventions, but
        // the issue is we cannot record it on the span until trailers have been received (in order
        // to get the gRPC status code). The current way the tower layer is setup does not allow us
        // to do this.
        // rpc.grpc.status_code = i32::from(Code::Unknown as u8),
        otel.kind = "client",
        otel.name = %path,
    )
}

/// This module manages a gRPC server-client connection over a Unix domain socket. Useful for unit testing
/// servers or clients within unit tests - supports parallelization within same process and
/// requires no port management.
///
/// Derived largely from <https://stackoverflow.com/a/71808401/> and
/// <https://github.com/hyperium/tonic/tree/master/examples/src/uds/>
#[cfg(test)]
pub(crate) mod uds_grpc_stream {
    use std::convert::Infallible;
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    use tokio::net::{UnixListener, UnixStream};
    use tokio_stream::wrappers::UnixListenerStream;
    use tonic::client::GrpcService;
    use tonic::server::NamedService;
    use tonic::transport::{Channel, Endpoint, Server, Uri};
    use tower::service_fn;

    /// The can be any valid URL. It is necessary to initialize an [`Endpoint`].
    #[allow(dead_code)]
    static FAUX_URL: &str = "http://api.example.rigetti.com";

    async fn build_uds_stream() -> Result<(UnixListenerStream, Channel), String> {
        let socket = NamedTempFile::new().unwrap();
        let socket = Arc::new(socket.into_temp_path());
        std::fs::remove_file(&*socket).unwrap();

        let uds = UnixListener::bind(&*socket).unwrap();
        let stream = UnixListenerStream::new(uds);

        let socket = Arc::clone(&socket);
        // Connect to the server over a Unix socket
        // The URL will be ignored.
        let channel = Endpoint::try_from(FAUX_URL)
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                let socket = Arc::clone(&socket);
                async move { UnixStream::connect(&*socket).await }
            }))
            .await
            .map_err(|e| format!("failed to connect to server: {e}"))?;

        Ok((stream, channel))
    }

    /// Serve the provide gRPC service over a Unix domain socket for the duration of the
    /// provided callback.
    pub(crate) async fn serve<S, F, R>(service: S, f: F) -> Result<(), String>
    where
        S: tower::Service<
                http::Request<hyper::body::Body>,
                Response = http::Response<tonic::body::BoxBody>,
                Error = Infallible,
            > + NamedService
            + GrpcService<tonic::body::BoxBody>
            + Clone
            + Send
            + 'static,
        F: FnOnce(Channel) -> R + Send,
        R: std::future::Future<Output = ()> + Send,
        <S as tower::Service<http::Request<hyper::Body>>>::Future: Send,
    {
        let (stream, channel) = build_uds_stream().await.unwrap();
        let serve_future = Server::builder()
            .add_service(service)
            .serve_with_incoming(stream);

        tokio::select! {
           result = serve_future => result.map_err(|e| format!("server unexpectedly exited with error: {e}")),
           () = f(channel) => Ok(()),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "tracing-opentelemetry")]
mod otel_tests {
    use opentelemetry::propagation::TextMapPropagator;
    use opentelemetry::trace::{TraceContextExt, TraceId};
    use opentelemetry_http::HeaderExtractor;
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use serde::{Deserialize, Serialize};
    use std::time::{Duration, SystemTime};
    use tonic::codegen::http::{HeaderMap, HeaderValue};
    use tonic::server::NamedService;
    use tonic::Request;
    use tonic_health::pb::health_check_response::ServingStatus;
    use tonic_health::pb::health_server::{Health, HealthServer};
    use tonic_health::{pb::health_client::HealthClient, server::HealthService};

    use crate::tonic::uds_grpc_stream;
    use qcs_api_client_common::configuration::AuthServer;
    use qcs_api_client_common::configuration::ClientConfiguration;
    use qcs_api_client_common::configuration::OAuthSession;
    use qcs_api_client_common::configuration::RefreshToken;

    use super::channel::{wrap_channel_with, wrap_channel_with_token_refresher};

    static HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

    /// Test that when tracing is enabled and no filter is set, any request is properly traced.
    #[tokio::test]
    async fn test_tracing_enabled_no_filter() {
        use qcs_api_client_common::tracing_configuration::TracingConfiguration;

        let tracing_configuration = TracingConfiguration::builder()
            .set_propagate_otel_context(true)
            .build();
        let client_config = ClientConfiguration::builder()
            .tracing_configuration(Some(tracing_configuration))
            .oauth_session(Some(OAuthSession::from_refresh_token(
                RefreshToken::new("refresh_token".to_string()),
                AuthServer::default(),
                Some(create_jwt()),
            )))
            .build()
            .expect("should be able to build client config");
        assert_grpc_health_check_traced(client_config).await;
    }

    /// Test that when tracing is enabled, no filter is set, and OTel context propagation is
    /// disabled, any request is properly traced without propagation.
    #[tokio::test]
    async fn test_tracing_enabled_no_filter_nor_otel_context_propagation() {
        use qcs_api_client_common::tracing_configuration::TracingConfiguration;

        let tracing_configuration = TracingConfiguration::default();
        let client_config = ClientConfiguration::builder()
            .tracing_configuration(Some(tracing_configuration))
            .oauth_session(Some(OAuthSession::from_refresh_token(
                RefreshToken::new("refresh_token".to_string()),
                AuthServer::default(),
                Some(create_jwt()),
            )))
            .build()
            .expect("failed to build client config");
        assert_grpc_health_check_traced(client_config).await;
    }

    /// Test that when tracing is enabled and the filter matches the gRPC request, the request is
    /// properly traced.
    #[tokio::test]
    async fn test_tracing_enabled_filter_passed() {
        use qcs_api_client_common::tracing_configuration::{TracingConfiguration, TracingFilter};

        let tracing_filter = TracingFilter::builder()
            .parse_strs_and_set_paths(&[HEALTH_CHECK_PATH])
            .expect("gRPC healthcheck path should be valid filter path")
            .build();

        let tracing_configuration = TracingConfiguration::builder()
            .set_filter(Some(tracing_filter))
            .set_propagate_otel_context(true)
            .build();

        let client_config = ClientConfiguration::builder()
            .tracing_configuration(Some(tracing_configuration))
            .oauth_session(Some(OAuthSession::from_refresh_token(
                RefreshToken::new("refresh_token".to_string()),
                AuthServer::default(),
                Some(create_jwt()),
            )))
            .build()
            .expect("failed to build client config");
        assert_grpc_health_check_traced(client_config).await;
    }

    /// Checks that the the [`RefreshService`] propagates the trace context via the traceparent metadata header and that the gRPC
    /// request span is properly created (ie the span duration is reasonable).
    #[allow(clippy::future_not_send)]
    async fn assert_grpc_health_check_traced(client_configuration: ClientConfiguration) {
        use opentelemetry::trace::FutureExt;

        let propagate_otel_context = client_configuration.tracing_configuration().is_some_and(
            qcs_api_client_common::tracing_configuration::TracingConfiguration::propagate_otel_context,
        );
        let spans = tracing_test::start(
            "test_trace_id_propagation",
            |trace_id, _span_id| async move {
                let sleepy_health_service = SleepyHealthService {
                    sleep_time: Duration::from_millis(50),
                };

                let interceptor = move |req| {
                    if propagate_otel_context {
                        validate_trace_id_propagated(trace_id, req)
                    } else {
                        validate_otel_context_not_propagated(req)
                    }
                };
                let health_server =
                    HealthServer::with_interceptor(sleepy_health_service, interceptor);

                uds_grpc_stream::serve(health_server, |channel| {
                    async {
                        let response = HealthClient::new(wrap_channel_with_token_refresher(
                            channel,
                            client_configuration,
                        ))
                        .check(Request::new(tonic_health::pb::HealthCheckRequest {
                            service: <HealthServer<HealthService> as NamedService>::NAME
                                .to_string(),
                        }))
                        .await
                        .unwrap();
                        assert_eq!(response.into_inner().status(), ServingStatus::Serving);
                    }
                    .with_current_context()
                })
                .await
                .unwrap();
            },
        )
        .await
        .unwrap();

        let grpc_span = spans
            .iter()
            .find(|span| span.name == *HEALTH_CHECK_PATH)
            .expect("failed to find gRPC span");
        let duration = grpc_span
            .end_time
            .duration_since(grpc_span.start_time)
            .expect("span should have valid timestamps");
        assert!(duration.as_millis() >= 50u128);
    }

    /// Test that when tracing is disabled, the request is not traced.
    #[tokio::test]
    async fn test_tracing_disabled() {
        let client_config = ClientConfiguration::builder()
            .oauth_session(Some(OAuthSession::from_refresh_token(
                RefreshToken::new("refresh_token".to_string()),
                AuthServer::default(),
                Some(create_jwt()),
            )))
            .build()
            .expect("should not fail to build client config");
        assert_grpc_health_check_not_traced(client_config).await;
    }

    /// Test that when tracing is enabled but the request does not match the configured filter, the
    /// request is not traced.
    #[tokio::test]
    async fn test_tracing_enabled_filter_not_passed() {
        use qcs_api_client_common::tracing_configuration::{TracingConfiguration, TracingFilter};

        let tracing_filter = TracingFilter::builder()
            .parse_strs_and_set_paths(&[HEALTH_CHECK_PATH])
            .expect("healthcheck path should be a valid filter path")
            .set_is_negated(true)
            .build();

        let tracing_configuration = TracingConfiguration::builder()
            .set_filter(Some(tracing_filter))
            .set_propagate_otel_context(true)
            .build();

        let client_config = ClientConfiguration::builder()
            .tracing_configuration(Some(tracing_configuration))
            .oauth_session(Some(OAuthSession::from_refresh_token(
                RefreshToken::new("refresh_token".to_string()),
                AuthServer::default(),
                Some(create_jwt()),
            )))
            .build()
            .expect("should be able to build client config");

        assert_grpc_health_check_not_traced(client_config.clone()).await;
    }

    /// Check that the traceparent metadata header is not set on the gRPC request and no tracing
    /// spans are produced for the gRPC request.
    #[allow(clippy::future_not_send)]
    async fn assert_grpc_health_check_not_traced(client_configuration: ClientConfiguration) {
        use opentelemetry::trace::FutureExt;

        let spans =
            tracing_test::start("test_tracing_disabled", |_trace_id, _span_id| async move {
                let interceptor = validate_otel_context_not_propagated;
                let health_server = HealthServer::with_interceptor(
                    SleepyHealthService {
                        sleep_time: Duration::from_millis(0),
                    },
                    interceptor,
                );

                uds_grpc_stream::serve(health_server, |channel| {
                    async {
                        let response =
                            HealthClient::new(wrap_channel_with(channel, client_configuration))
                                .check(Request::new(tonic_health::pb::HealthCheckRequest {
                                    service: <HealthServer<HealthService> as NamedService>::NAME
                                        .to_string(),
                                }))
                                .await
                                .unwrap();
                        assert_eq!(response.into_inner().status(), ServingStatus::Serving);
                    }
                    .with_current_context()
                })
                .await
                .unwrap();
            })
            .await
            .unwrap();

        assert!(spans.iter().all(|span| { span.name != *HEALTH_CHECK_PATH }));
    }

    const JWT_SECRET: &[u8] = b"top-secret";

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: u64,
    }

    /// Create an HS256 signed JWT token with sub and exp claims. This is good enough to pass the
    /// [`RefreshService`] token validation.
    pub(crate) fn create_jwt() -> String {
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
        let expiration = SystemTime::now()
            .checked_add(Duration::from_secs(60))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            sub: "test-uid".to_string(),
            exp: expiration,
        };
        // The client doesn't check the signature, so for convenience here, we just sign with HS256
        // instead of RS256.
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
    }

    #[derive(Debug, thiserror::Error)]
    #[allow(variant_size_differences)]
    enum ServerAssertionError {
        #[error("trace id did not propagate to server: {0}")]
        UnexpectedTraceId(String),
        #[error("otel context headers unexpectedly sent to server")]
        UnexpectedOTelContextHeaders,
    }

    impl From<ServerAssertionError> for tonic::Status {
        fn from(server_assertion_error: ServerAssertionError) -> Self {
            Self::invalid_argument(server_assertion_error.to_string())
        }
    }

    /// Given an incoming gRPC request, validate that the the specified [`TraceId`] is propagated
    /// via the `traceparent` metadata header.
    fn validate_trace_id_propagated(
        trace_id: TraceId,
        req: Request<()>,
    ) -> Result<Request<()>, tonic::Status> {
        req.metadata()
            .get("traceparent")
            .ok_or_else(|| {
                ServerAssertionError::UnexpectedTraceId(
                    "request traceparent metadata not present".to_string(),
                )
            })
            .and_then(|traceparent| {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "traceparent",
                    HeaderValue::from_str(traceparent.to_str().map_err(|_| {
                        ServerAssertionError::UnexpectedTraceId(
                            "failed to deserialize trace parent".to_string(),
                        )
                    })?)
                    .map_err(|_| {
                        ServerAssertionError::UnexpectedTraceId(
                            "failed to serialize trace parent as HeaderValue".to_string(),
                        )
                    })?,
                );
                Ok(headers)
            })
            .and_then(|headers| {
                let extractor = HeaderExtractor(&headers);
                let propagator = TraceContextPropagator::new();
                let context = propagator.extract(&extractor);
                let propagated_trace_id = context.span().span_context().trace_id();
                if propagated_trace_id == trace_id {
                    Ok(req)
                } else {
                    Err(ServerAssertionError::UnexpectedTraceId(format!(
                        "expected trace id {trace_id}, got {propagated_trace_id}",
                    )))
                }
            })
            .map_err(Into::into)
    }

    /// Simply validate that the `traceparent` and `tracestate` metadata headers are not present
    /// on the incoming gRPC.
    fn validate_otel_context_not_propagated(
        req: Request<()>,
    ) -> Result<Request<()>, tonic::Status> {
        if req.metadata().get("traceparent").is_some() || req.metadata().get("tracestate").is_some()
        {
            Err(ServerAssertionError::UnexpectedOTelContextHeaders.into())
        } else {
            Ok(req)
        }
    }

    /// An implementation of the gRPC [`HealthService`] that sleeps for the configured duration on before returning a response.
    /// This is useful for making assertions on span durations. It is also necessary in order to
    /// wrap the [`HealthServer`] in an interceptor, which is not possible with public methods in
    /// the health service crate.
    ///
    /// Derived in part from <https://github.com/hyperium/tonic/blob/master/tonic-health/src/generated/grpc.health.v1.rs/>
    #[derive(Clone)]
    struct SleepyHealthService {
        sleep_time: Duration,
    }

    #[tonic::async_trait]
    impl Health for SleepyHealthService {
        async fn check(
            &self,
            _request: Request<tonic_health::pb::HealthCheckRequest>,
        ) -> Result<tonic::Response<tonic_health::pb::HealthCheckResponse>, tonic::Status> {
            tokio::time::sleep(self.sleep_time).await;
            let response = tonic_health::pb::HealthCheckResponse {
                status: ServingStatus::Serving as i32,
            };
            Ok(tonic::Response::new(response))
        }

        type WatchStream = tokio_stream::wrappers::ReceiverStream<
            Result<tonic_health::pb::HealthCheckResponse, tonic::Status>,
        >;

        async fn watch(
            &self,
            _request: Request<tonic_health::pb::HealthCheckRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status> {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let response = tonic_health::pb::HealthCheckResponse {
                status: ServingStatus::Serving as i32,
            };
            tx.send(Ok(response)).await.unwrap();
            Ok(tonic::Response::new(
                tokio_stream::wrappers::ReceiverStream::new(rx),
            ))
        }
    }

    /// We need a single global ``SpanProcessor`` because these tests have to work using
    /// ``opentelemetry::global`` and ``tracing_subscriber::set_global_default``. Otherwise,
    /// we can't make guarantees about where the spans are processed and therefore could
    /// not make assertions about the traced spans.
    mod tracing_test {
        use futures_util::Future;
        use opentelemetry::global::BoxedTracer;
        use opentelemetry::trace::{
            mark_span_as_active, FutureExt, Span, SpanId, TraceId, TraceResult, Tracer,
            TracerProvider,
        };
        use opentelemetry_sdk::export::trace::SpanData;
        use opentelemetry_sdk::trace::SpanProcessor;
        use std::collections::HashMap;
        use std::sync::{Arc, RwLock};

        /// Start a new test span and run the specified callback with the span as the active span.
        /// The call back is provided the span and trace ids. At the end of the callback, we wait
        /// for the span to be processed and then return all of the spans that were processed for
        /// this particular test.
        #[allow(clippy::future_not_send)]
        pub(crate) async fn start<F, R>(name: &'static str, f: F) -> Result<Vec<SpanData>, String>
        where
            F: FnOnce(TraceId, SpanId) -> R + Send,
            R: Future<Output = ()> + Send,
        {
            let tracer = CacheProcessor::tracer();
            let span = tracer.start(name);
            let span_id = span.span_context().span_id();
            let trace_id = span.span_context().trace_id();
            let cache = CACHE
                .get()
                .expect("cache should be initialized with cache tracer");
            cache.subscribe(span_id)?;
            async {
                let _guard = mark_span_as_active(span);
                f(trace_id, span_id).with_current_context().await;
            }
            .await;

            // wait for test span to be processed.
            cache.notified(span_id).await?;

            // remove and return the spans processed for this test.
            let mut data = cache.data.write().map_err(|e| e.to_string())?;
            Ok(data.remove(&trace_id).unwrap_or_default())
        }

        static CACHE: once_cell::sync::OnceCell<CacheProcessor> = once_cell::sync::OnceCell::new();

        #[derive(Debug, Clone, Default)]
        struct CacheProcessor {
            data: Arc<RwLock<HashMap<TraceId, Vec<SpanData>>>>,
            notifications: Arc<RwLock<HashMap<SpanId, Arc<tokio::sync::Notify>>>>,
        }

        impl CacheProcessor {
            /// Initializes the [`CACHE`] and sets the global `opentelemetry::global::tracer_provider` and `tracing_subscriber::global_default`.
            /// These initializations occur safely behind a `OnceCell` initialization, so they can be used by several tests.
            fn tracer() -> BoxedTracer {
                use tracing_subscriber::layer::SubscriberExt;

                CACHE.get_or_init(|| {
                    let processor = Self::default();
                    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
                        .with_span_processor(processor.clone())
                        .build();
                    opentelemetry::global::set_tracer_provider(provider.clone());
                    let tracer = provider.tracer("test_channel");
                    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
                    let subscriber = tracing_subscriber::Registry::default().with(telemetry);
                    tracing::subscriber::set_global_default(subscriber)
                        .expect("tracing subscriber already set");
                    processor
                });
                opentelemetry::global::tracer("test_channel")
            }

            /// Ensure that a [`Notification`] exists for the provided span id.
            fn subscribe(&self, span_id: SpanId) -> Result<(), String> {
                let notify = Arc::new(tokio::sync::Notify::new());
                self.notifications
                    .write()
                    .map_err(|e| e.to_string())?
                    .insert(span_id, notify);
                Ok(())
            }

            /// Wait for the specified [`SpanId`] to be processed.
            async fn notified(&self, span_id: SpanId) -> Result<(), String> {
                let notify = {
                    let notifications = self.notifications.read().map_err(|e| format!("{e}"))?;
                    notifications
                        .get(&span_id)
                        .ok_or("span notification never subscribed")?
                        .clone()
                };
                notify.notified().await;
                Ok(())
            }
        }

        impl SpanProcessor for CacheProcessor {
            fn on_start(
                &self,
                _span: &mut opentelemetry_sdk::trace::Span,
                _cx: &opentelemetry::Context,
            ) {
            }

            fn on_end(&self, span: SpanData) {
                let trace_id = span.span_context.trace_id();
                let span_id = span.span_context.span_id();
                {
                    let mut data = self
                        .data
                        .write()
                        .expect("failed to write access cache span data");
                    if let Some(spans) = data.get_mut(&trace_id) {
                        spans.push(span);
                    } else {
                        data.insert(trace_id, vec![span]);
                    }
                }

                if let Some(notify) = self
                    .notifications
                    .read()
                    .expect("failed to read access span notifications during span processing")
                    .get(&span_id)
                {
                    notify.notify_waiters();
                }
            }

            /// This is a no-op because spans are processed synchronously in `on_end`.
            fn force_flush(&self) -> TraceResult<()> {
                Ok(())
            }

            fn shutdown(&mut self) -> TraceResult<()> {
                Ok(())
            }
        }
    }
}
