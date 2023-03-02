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
use http_body::Full;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::client::GrpcService;
use tonic::codegen::http::uri::InvalidUri;
use tonic::codegen::http::{Request, Response, StatusCode};
use tonic::codegen::Body;
use tonic::transport::{Channel, Error as TransportError, Uri};
use tonic::Status;
use tower::{Layer, ServiceBuilder};

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
    /// The provided access token is not a valid header value.
    #[error("access token is not a valid header value: {0}")]
    InvalidAccessToken(#[source] http::header::InvalidHeaderValue),
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
    Ok(wrap_channel_with(
        channel,
        ClientConfiguration::load_default().await?,
    ))
}

/// Set up the given [`Channel`] with QCS authentication.
///
/// # Errors
///
/// See [`Error`]
pub async fn wrap_channel_with_profile(
    channel: Channel,
    profile: String,
) -> Result<RefreshService<Channel>, Error> {
    Ok(wrap_channel_with(
        channel,
        ClientConfiguration::load_profile(profile).await?,
    ))
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
        let config = ClientConfiguration::load_default().await?;
        Ok(Self::with_config(config))
    }

    pub async fn with_profile(profile: String) -> Result<Self, Error> {
        let config = ClientConfiguration::load_profile(profile).await?;
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
    let bytes =
        Full::from(bytes).map_err(|_| Status::internal("this will never happen from Infallible"));
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
) -> Result<Response<<Channel as GrpcService<BoxBody>>::ResponseBody>, Error> {
    let header_val = format!("Bearer {token}")
        .try_into()
        .map_err(Error::InvalidAccessToken)?;
    request.headers_mut().insert("authorization", header_val);
    service.call(request).await.map_err(Error::from)
}

#[cfg(feature = "otel-tracing")]
async fn make_traced_request(
    service: &mut Channel,
    mut request: Request<BoxBody>,
    token: String,
) -> Result<Response<<Channel as GrpcService<BoxBody>>::ResponseBody>, Error> {
    let header_val = format!("Bearer {token}")
        .try_into()
        .map_err(Error::InvalidAccessToken)?;
    request.headers_mut().insert("authorization", header_val);

    use opentelemetry::trace::FutureExt;
    use tracing::Instrument;
    let span = make_grpc_request_span(&request);
    service
        .call(request)
        .with_current_context()
        .instrument(span)
        .await
        .map_err(Error::from)
}

impl GrpcService<BoxBody> for RefreshService<Channel>
where
    Channel: GrpcService<BoxBody, Error = TransportError> + Clone + Send + 'static,
    <Channel as GrpcService<BoxBody>>::Future: Send,
{
    type ResponseBody = <Channel as GrpcService<BoxBody>>::ResponseBody;
    type Error = Error;
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
        let config = self.config.clone();
        Box::pin(async move {
            #[cfg(feature = "otel-tracing")]
            use opentelemetry_api::trace::FutureExt;

            #[cfg(feature = "otel-tracing")]
            return traced_service_call(req, config, service)
                .with_current_context()
                .await;

            #[cfg(not(feature = "otel-tracing"))]
            return service_call(req, config, service).await;
        })
    }
}

async fn service_call(
    req: Request<BoxBody>,
    config: ClientConfiguration,
    mut channel: Channel,
) -> Result<Response<<Channel as GrpcService<BoxBody>>::ResponseBody>, Error> {
    let token = config.get_bearer_access_token().await?;
    let (req, retry_req) = clone_request(req).await;
    let resp = make_request(&mut channel, req, token).await?;
    match resp.status() {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            // Refresh token and try again
            let token = config.refresh().await?;
            make_request(&mut channel, retry_req, token).await
        }
        _ => Ok(resp),
    }
}

#[cfg(feature = "otel-tracing")]
async fn traced_service_call(
    original_req: Request<BoxBody>,
    config: ClientConfiguration,
    mut channel: Channel,
) -> Result<Response<<Channel as GrpcService<BoxBody>>::ResponseBody>, Error> {
    use opentelemetry::{propagation::TextMapPropagator, sdk::propagation::TraceContextPropagator};
    use opentelemetry_api::trace::FutureExt;
    use opentelemetry_http::HeaderInjector;
    use urlpattern::UrlPatternMatchInput;

    // The request URI here doesn't include the base url, so we have  to manually add it here to evaluate request filter patterns.
    let full_request_url = format!(
        "{}{}",
        config.grpc_api_url(),
        &original_req.uri().to_string()
    );

    if !config
        .tracing_configuration()
        .map(|tracing_config| {
            tracing_config
                .filter()
                .map(|filter| {
                    url::Url::parse(full_request_url.as_str())
                        .map(UrlPatternMatchInput::Url)
                        .map(|input| filter.is_enabled(&input))
                        // tracing enabled and filter set, but url failed to parse for filter evaluation - do not trace the request
                        .unwrap_or(false)
                })
                // tracing enabled, but not filter set, trace all requests
                .unwrap_or(true)
        })
        // tracing not enabled, do not trace the request
        .unwrap_or(false)
    {
        return service_call(original_req, config, channel).await;
    }

    let token = config
        .get_bearer_access_token()
        .with_current_context()
        .await?;
    let (mut req, mut retry_req) = clone_request(original_req).with_current_context().await;

    // Poor semantics here, but adding custom gRPC metadata is equivalent to setting request
    // headers: https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/doc/PROTOCOL-HTTP2.md.
    let propagator = TraceContextPropagator::new();
    let mut injector = HeaderInjector(req.headers_mut());
    propagator.inject_context(&opentelemetry::Context::current(), &mut injector);

    let resp = make_traced_request(&mut channel, req, token)
        .with_current_context()
        .await?;

    match resp.status() {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            tracing::info!("refreshing token after receiving unauthorized or forbidden status",);
            // Refresh token and try again
            let token = config.refresh().with_current_context().await?;
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

#[cfg(feature = "otel-tracing")]
static GRPC_SPAN_NAME: &str = "gRPC request";

/// Creates a gRPC request span that conforms to the gRPC semantic conventions. See <https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/rpc.md> for details.
#[cfg(feature = "otel-tracing")]
fn make_grpc_request_span(
    request: &tonic::codegen::http::Request<tonic::body::BoxBody>,
) -> tracing::Span {
    let _method = request.method();
    let url = request.uri();
    let path = url.path();
    let mut path_split = path.split('/');
    let (_, service, method) = (path_split.next(), path_split.next(), path_split.next());
    let service = service.unwrap_or("");
    let method = method.unwrap_or("");
    let _scheme = url.scheme();
    let host = url.host().unwrap_or("");
    let host_port = url.port().map(|p| p.as_u16()).unwrap_or(0u16);
    tracing::span!(
        tracing::Level::TRACE,
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

#[cfg(test)]
#[cfg(feature = "otel-tracing")]
mod tests {
    use opentelemetry::propagation::TextMapPropagator;
    use opentelemetry::sdk::propagation::TraceContextPropagator;
    use opentelemetry::trace::{TraceContextExt, TraceId};
    use opentelemetry_http::HeaderExtractor;
    use qcs_api_client_common::configuration::Tokens;
    use qcs_api_client_common::ClientConfiguration;
    use serde::{Deserialize, Serialize};
    use std::time::{Duration, SystemTime};
    use tonic::codegen::http::{HeaderMap, HeaderValue};
    use tonic::transport::NamedService;
    use tonic::Request;
    use tonic_health::proto::health_check_response::ServingStatus;
    use tonic_health::proto::health_server::{Health, HealthServer};
    use tonic_health::{proto::health_client::HealthClient, server::HealthService};

    use super::wrap_channel_with;

    static HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

    /// Test that when tracing is enabled and no filter is set, any request is properly traced.
    #[tokio::test]
    async fn test_tracing_enabled_no_filter() {
        use qcs_api_client_common::otel_tracing::TracingConfiguration;

        let client_config = ClientConfiguration::builder()
            .set_tracing_configuration(Some(TracingConfiguration::default()))
            .set_tokens(Tokens {
                bearer_access_token: Some(create_jwt()),
                refresh_token: Some("refresh_token".to_string()),
            })
            .build()
            .expect("failed to build client config");
        assert_grpc_health_check_traced(client_config).await;
    }

    /// Test that when tracing is enabled and the filter matches the gRPC request, the request is
    /// properly traced.
    #[tokio::test]
    async fn test_tracing_enabled_filter_passed() {
        use qcs_api_client_common::otel_tracing::{TracingConfiguration, TracingFilter};

        let tracing_filter = TracingFilter::builder()
            .parse_strs_and_set_paths(&[HEALTH_CHECK_PATH])
            .expect("gRPC healthcheck path should be valid filter path")
            .build();

        let tracing_configuration = TracingConfiguration::builder()
            .set_filter(Some(tracing_filter))
            .build();

        let client_config = ClientConfiguration::builder()
            .set_tracing_configuration(Some(tracing_configuration))
            .set_tokens(Tokens {
                bearer_access_token: Some(create_jwt()),
                refresh_token: Some("refresh_token".to_string()),
            })
            .build()
            .expect("failed to build client config");
        assert_grpc_health_check_traced(client_config).await;
    }

    /// Checks that the the [`RefreshService`] propagates the trace context via the traceparent metadata header and that the gRPC
    /// request span is properly created (ie the span duration is reasonable).
    async fn assert_grpc_health_check_traced(client_configuration: ClientConfiguration) {
        use opentelemetry::trace::FutureExt;

        let spans = tracing_test::start(
            "test_trace_id_propagation",
            |trace_id, _span_id| async move {
                let interceptor = move |req| validate_trace_id_propagated(trace_id, req);
                let health_server = HealthServer::with_interceptor(
                    SleepyHealthService {
                        sleep_time: Duration::from_millis(50),
                    },
                    interceptor,
                );

                uds_grpc_stream::serve(health_server, |channel| {
                    async {
                        let mut client =
                            HealthClient::new(wrap_channel_with(channel, client_configuration));
                        let response =
                            client.check(Request::new(tonic_health::proto::HealthCheckRequest {
                                service: <HealthServer<HealthService> as NamedService>::NAME
                                    .to_string(),
                            }));
                        let response = response.await.unwrap();
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
            .set_tokens(Tokens {
                bearer_access_token: Some(create_jwt()),
                refresh_token: Some("refresh_token".to_string()),
            })
            .build()
            .expect("failed to build client config");
        assert_grpc_health_check_not_traced(client_config).await;
    }

    /// Test that when tracing is enabled but the request does not match the configured filter, the
    /// request is not traced.
    #[tokio::test]
    async fn test_tracing_enabled_filter_not_passed() {
        use qcs_api_client_common::otel_tracing::{TracingConfiguration, TracingFilter};

        let tracing_filter = TracingFilter::builder()
            .parse_strs_and_set_paths(&[HEALTH_CHECK_PATH])
            .expect("healthcheck path should be a valid filter path")
            .set_is_negated(true)
            .build();

        let tracing_configuration = TracingConfiguration::builder()
            .set_filter(Some(tracing_filter))
            .build();

        let client_config = ClientConfiguration::builder()
            .set_tracing_configuration(Some(tracing_configuration))
            .set_tokens(Tokens {
                bearer_access_token: Some(create_jwt()),
                refresh_token: Some("refresh_token".to_string()),
            })
            .build()
            .expect("failed to build client config");

        assert_grpc_health_check_not_traced(client_config.to_owned()).await;
    }

    /// Check that the traceparent metadata header is not set on the gRPC request and no tracing
    /// spans are produced for the gRPC request.
    async fn assert_grpc_health_check_not_traced(client_configuration: ClientConfiguration) {
        use opentelemetry::trace::FutureExt;

        let spans =
            tracing_test::start("test_tracing_disabled", |_trace_id, _span_id| async move {
                let interceptor = validate_trace_parent_not_propagated;
                let health_server = HealthServer::with_interceptor(
                    SleepyHealthService {
                        sleep_time: Duration::from_millis(0),
                    },
                    interceptor,
                );

                uds_grpc_stream::serve(health_server, |channel| {
                    async {
                        let mut client =
                            HealthClient::new(wrap_channel_with(channel, client_configuration));
                        let response =
                            client.check(Request::new(tonic_health::proto::HealthCheckRequest {
                                service: <HealthServer<HealthService> as NamedService>::NAME
                                    .to_string(),
                            }));
                        let response = response.await.unwrap();
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
        exp: usize,
    }

    /// Create an HS256 signed JWT token with sub and exp claims. This is good enough to pass the
    /// [`RefreshService`] token validation.
    pub fn create_jwt() -> String {
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
        let expiration = std::time::SystemTime::now()
            .checked_add(Duration::from_secs(60))
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = Claims {
            sub: "test-uid".to_string(),
            exp: expiration as usize,
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
        #[error("traceparent unexpectedly propagated to server")]
        UnexpectedTraceParent,
    }

    impl From<ServerAssertionError> for tonic::Status {
        fn from(server_assertion_error: ServerAssertionError) -> tonic::Status {
            tonic::Status::invalid_argument(server_assertion_error.to_string())
        }
    }

    /// Given an incoming gRPC request, validate that the the specified [`TraceId`] is propagated
    /// via the `traceparent` metadata header.
    fn validate_trace_id_propagated(
        trace_id: TraceId,
        req: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
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
                        "expected trace id {}, got {}",
                        trace_id, propagated_trace_id
                    )))
                }
            })
            .map_err(Into::into)
    }

    /// Simply validate that the `traceparent` metadata header is not present on the incoming gRPC.
    fn validate_trace_parent_not_propagated(
        req: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        match req.metadata().get("traceparent") {
            Some(_) => Err(ServerAssertionError::UnexpectedTraceParent.into()),
            None => Ok(req),
        }
    }

    /// An implementation of the gRPC [`HealthService`] that sleeps for the configured duration on before returning a response.
    /// This is useful for making assertions on span durations. It is also necessary in order to
    /// wrap the [`HealthServer`] in an interceptor, which is not possible with public methods in
    /// the health service crate.
    ///
    /// Derived in part from https://github.com/hyperium/tonic/blob/master/tonic-health/src/generated/grpc.health.v1.rs
    #[derive(Clone)]
    struct SleepyHealthService {
        sleep_time: Duration,
    }

    #[tonic::async_trait]
    impl Health for SleepyHealthService {
        async fn check(
            &self,
            _request: Request<tonic_health::proto::HealthCheckRequest>,
        ) -> Result<tonic::Response<tonic_health::proto::HealthCheckResponse>, tonic::Status>
        {
            tokio::time::sleep(self.sleep_time).await;
            let response = tonic_health::proto::HealthCheckResponse {
                status: ServingStatus::Serving as i32,
            };
            Ok(tonic::Response::new(response))
        }

        type WatchStream = tokio_stream::wrappers::ReceiverStream<
            Result<tonic_health::proto::HealthCheckResponse, tonic::Status>,
        >;

        async fn watch(
            &self,
            _request: Request<tonic_health::proto::HealthCheckRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status> {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            let response = tonic_health::proto::HealthCheckResponse {
                status: ServingStatus::Serving as i32,
            };
            tx.send(Ok(response)).await.unwrap();
            Ok(tonic::Response::new(
                tokio_stream::wrappers::ReceiverStream::new(rx),
            ))
        }
    }

    /// This module manages a gRPC server-client connection over a Unix domain socket. Useful for unit testing
    /// servers or clients within unit tests - supports parallelization within same process and
    /// requires no port management.
    ///
    /// Derived largely from  https://stackoverflow.com/a/71808401 and
    /// https://github.com/hyperium/tonic/tree/master/examples/src/uds
    mod uds_grpc_stream {
        use std::convert::Infallible;
        use std::sync::Arc;
        use tempfile::NamedTempFile;
        use tokio::net::{UnixListener, UnixStream};
        use tokio_stream::wrappers::UnixListenerStream;
        use tonic::transport::{Channel, Endpoint, NamedService, Server, Uri};
        use tower::service_fn;

        /// The can be any valid URL. It is necessary to initialize an [`Endpoint`].
        #[allow(dead_code)]
        static FAUX_URL: &str = "http://api.example.com";

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
                .map_err(|e| format!("failed to connect to server: {}", e))?;

            Ok((stream, channel))
        }

        /// Serve the provide gRPC service over a Unix domain socket for the duration of the
        /// provided callback.
        pub async fn serve<S, F, R>(service: S, f: F) -> Result<(), String>
        where
            S: tower::Service<
                    http::Request<hyper::body::Body>,
                    Response = http::Response<tonic::body::BoxBody>,
                    Error = Infallible,
                > + NamedService
                + Clone
                + Send
                + 'static,
            S::Future: Send + 'static,
            F: FnOnce(Channel) -> R,
            R: std::future::Future<Output = ()>,
        {
            let (stream, channel) = build_uds_stream().await.unwrap();
            let serve_future = Server::builder()
                .add_service(service)
                .serve_with_incoming(stream);

            tokio::select! {
               result = serve_future => result.map_err(|e| format!("server unexpectedly exited with error: {}", e)),
               _ = f(channel) => Ok(()),
            }
        }
    }

    /// We need a single global SpanProcessor because these tests have to work using
    /// opentelemetry::global and tracing_subscriber::set_global_default. Otherwise,
    /// we can't make guarantees about where the spans are processed and therefore could
    /// not make assertions about the traced spans.
    mod tracing_test {
        use futures_util::Future;
        use opentelemetry::global::BoxedTracer;
        use opentelemetry::sdk::export::trace::SpanData;
        use opentelemetry::trace::{
            mark_span_as_active, FutureExt, Span, SpanId, TraceId, TraceResult, Tracer,
            TracerProvider,
        };
        use opentelemetry_sdk::trace::SpanProcessor;
        use std::collections::HashMap;
        use std::sync::{Arc, RwLock};

        /// Start a new test span and run the specified callback with the span as the active span.
        /// The call back is provided the span and trace ids. At the end of the callback, we wait
        /// for the span to be processed and then return all of the spans that were processed for
        /// this particular test.
        pub async fn start<F, R>(name: &'static str, f: F) -> Result<Vec<SpanData>, String>
        where
            F: FnOnce(TraceId, SpanId) -> R,
            R: Future<Output = ()>,
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
                    let provider = opentelemetry::sdk::trace::TracerProvider::builder()
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
                    let notifications = self.notifications.read().map_err(|e| format!("{}", e))?;
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
                _span: &mut opentelemetry::sdk::trace::Span,
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
