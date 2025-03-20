use std::convert::Infallible;

/// QCS Middleware for [`tonic`] clients.
///
use futures_util::pin_mut;
use http::Request;
use http_body::Body;
use http_body_util::BodyExt;
use tonic::body::BoxBody;

mod channel;
mod common;
mod error;
#[cfg(feature = "grpc-web")]
mod grpc_web;
mod refresh;
mod retry;
#[cfg(feature = "tracing")]
mod trace;

pub use channel::*;
pub use error::*;
#[cfg(feature = "grpc-web")]
pub use grpc_web::*;
pub use refresh::*;
pub use retry::*;
#[cfg(feature = "tracing")]
pub use trace::*;

/// An error observed while duplicating a request body. This may be returned by any
/// [`tower::Service`] that duplicates a request body for the purpose of retrying a request.
#[derive(Debug, thiserror::Error)]
pub enum RequestBodyDuplicationError {
    /// The inner service returned an error from the server, or the client cancelled the
    /// request.
    #[error(transparent)]
    Status(#[from] tonic::Status),
    /// Failed to read the request body for cloning.
    #[error("failed to read request body for request clone: {0}")]
    HttpBody(#[from] http::Error),
}

impl From<RequestBodyDuplicationError> for tonic::Status {
    fn from(err: RequestBodyDuplicationError) -> tonic::Status {
        match err {
            RequestBodyDuplicationError::Status(status) => status,
            RequestBodyDuplicationError::HttpBody(error) => tonic::Status::cancelled(format!(
                "failed to read request body for request clone: {error}"
            )),
        }
    }
}

type RequestBodyDuplicationResult<T> = Result<T, RequestBodyDuplicationError>;

/// This function should only be used with Unary requests; Stream requests are
/// untested. It eagerly collects all request data into a buffer, consuming the
/// original stream. Additionally, it assumes that all frames are data frames
/// (i.e. the stream cannot contain any trailers); if a trailer frame is found,
/// the cancelled status will be returned.
async fn build_duplicate_frame_bytes(
    mut request: Request<tonic::body::BoxBody>,
) -> RequestBodyDuplicationResult<(tonic::body::BoxBody, tonic::body::BoxBody)> {
    let mut bytes = Vec::new();

    let body = request.body_mut();
    pin_mut!(body);
    while let Some(result) = std::future::poll_fn(|cx| body.as_mut().poll_frame(cx)).await {
        let frame_bytes = result?.into_data().map_err(|frame| {
            tonic::Status::cancelled(format!(
                "cannot duplicate a frame that is not a data frame: {frame:?}"
            ))
        })?;
        bytes.extend(frame_bytes);
    }

    let bytes = http_body_util::Full::from(bytes)
        .map_err(|_: Infallible| -> tonic::Status { unreachable!() });
    Ok((
        tonic::body::BoxBody::new(bytes.clone()),
        tonic::body::BoxBody::new(bytes),
    ))
}

/// This function should only be used with Unary requests; Stream requests are
/// untested. See comment on `build_duplicate_frame_bytes`.
async fn build_duplicate_request(
    req: Request<BoxBody>,
) -> RequestBodyDuplicationResult<(Request<BoxBody>, Request<BoxBody>)> {
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

    let (body_1, body_2) = build_duplicate_frame_bytes(req).await?;

    let req_1 = builder_1.body(body_1)?;

    let req_2 = builder_2.body(body_2)?;

    Ok((req_1, req_2))
}

/// This module manages a gRPC server-client connection over a Unix domain socket. Useful for unit testing
/// servers or clients within unit tests - supports parallelization within same process and
/// requires no port management.
///
/// Derived largely from <https://stackoverflow.com/a/71808401> and
/// <https://github.com/hyperium/tonic/tree/master/examples/src/uds>.
#[cfg(test)]
pub(crate) mod uds_grpc_stream {
    use hyper_util::rt::TokioIo;
    use opentelemetry::trace::FutureExt;
    use std::convert::Infallible;
    use tempfile::TempDir;
    use tokio::net::UnixStream;
    use tokio_stream::wrappers::UnixListenerStream;
    use tonic::server::NamedService;
    use tonic::transport::{Channel, Endpoint, Server};

    /// The can be any valid URL. It is necessary to initialize an [`Endpoint`].
    #[allow(dead_code)]
    static FAUX_URL: &str = "http://api.example.rigetti.com";

    fn build_server_stream(path: String) -> Result<UnixListenerStream, Error> {
        let uds =
            tokio::net::UnixListener::bind(path.clone()).map_err(|_| Error::BindUnixPath(path))?;
        Ok(UnixListenerStream::new(uds))
    }

    async fn build_client_channel(path: String) -> Result<Channel, Error> {
        let connector = tower::service_fn(move |_: tonic::transport::Uri| {
            let path = path.clone();
            async move {
                let connection = UnixStream::connect(path).await?;
                Ok::<_, std::io::Error>(TokioIo::new(connection))
            }
        });
        let channel = Endpoint::try_from(FAUX_URL)
            .map_err(|source| Error::Endpoint {
                url: FAUX_URL.to_string(),
                source,
            })?
            .connect_with_connector(connector)
            .await
            .map_err(|source| Error::Connect { source })?;
        Ok(channel)
    }

    /// Errors when connecting a server-client [`Channel`] over a Unix domain socket for testing gRPC
    /// services.
    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        /// Failed to initialize an [`Endpoint`] for the provided URL.
        #[error("failed to initialize endpoint for {url}: {source}")]
        Endpoint {
            /// The URL that failed to initialize.
            url: String,
            /// The source of the error.
            #[source]
            source: tonic::transport::Error,
        },
        /// Failed to connect to the provided endpoint.
        #[error("failed to connect to endpoint: {source}")]
        Connect {
            /// The source of the error.
            #[source]
            source: tonic::transport::Error,
        },
        /// Failed to bind the provided path as a Unix domain socket.
        #[error("failed to bind path as unix listener: {0}")]
        BindUnixPath(String),
        /// Failed to initialize a temporary file for the Unix domain socket.
        #[error("failed in initialize tempfile: {0}")]
        TempFile(#[from] std::io::Error),
        /// Failed to convert the tempfile to an [`OsString`].
        #[error("failed to convert tempfile to OsString")]
        TempFileOsString,
        /// Failed to bind to the Unix domain socket.
        #[error("failed to bind to UDS: {0}")]
        TonicTransport(#[from] tonic::transport::Error),
    }

    /// Serve the provided gRPC service over a Unix domain socket for the duration of the
    /// provided callback.
    ///
    /// # Errors
    ///
    /// See [`Error`].
    #[allow(clippy::significant_drop_tightening)]
    pub async fn serve<S, F, R>(service: S, f: F) -> Result<(), Error>
    where
        S: tower::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<tonic::body::BoxBody>,
                Error = Infallible,
            > + NamedService
            + Clone
            + Send
            + 'static,
        S::Future: Send,
        F: FnOnce(Channel) -> R + Send,
        R: std::future::Future<Output = ()> + Send,
    {
        let directory = TempDir::new()?;
        let file = directory.path().as_os_str();
        let file = file.to_os_string();
        let file = file.into_string().map_err(|_| Error::TempFileOsString)?;
        let file = format!("{file}/test.sock");
        let stream = build_server_stream(file.clone())?;

        let channel = build_client_channel(file).await?;
        let serve_future = Server::builder()
            .add_service(service)
            .serve_with_incoming(stream);

        tokio::select! {
           result = serve_future => result.map_err(Error::TonicTransport),
           () = f(channel).with_current_context() => Ok(()),
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

    use crate::tonic::{uds_grpc_stream, wrap_channel_with_tracing};
    use qcs_api_client_common::configuration::ClientConfiguration;
    use qcs_api_client_common::configuration::{AuthServer, OAuthSession, RefreshToken};

    static HEALTH_CHECK_PATH: &str = "/grpc.health.v1.Health/Check";

    const FAUX_BASE_URL: &str = "http://api.example.rigetti.com";

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
                        let response = HealthClient::new(wrap_channel_with_tracing(
                            channel,
                            FAUX_BASE_URL.to_string(),
                            client_configuration
                                .tracing_configuration()
                                .unwrap()
                                .clone(),
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

        let status_code_attribute =
            tracing_test::get_span_attribute(grpc_span, "rpc.grpc.status_code")
                .expect("gRPC span should have status code attribute");
        assert_eq!(status_code_attribute, (tonic::Code::Ok as u8).to_string());
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
                        let response = HealthClient::new(wrap_channel_with_tracing(
                            channel,
                            FAUX_BASE_URL.to_string(),
                            client_configuration
                                .tracing_configuration()
                                .unwrap()
                                .clone(),
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

            fn shutdown(&self) -> TraceResult<()> {
                Ok(())
            }
        }

        /// Get the Opentelemetry attribute value for the provided key.
        #[must_use]
        pub(super) fn get_span_attribute(
            span_data: &SpanData,
            key: &'static str,
        ) -> Option<String> {
            span_data
                .attributes
                .iter()
                .find(|attr| attr.key.to_string() == *key)
                .map(|kv| kv.value.to_string())
        }
    }
}
