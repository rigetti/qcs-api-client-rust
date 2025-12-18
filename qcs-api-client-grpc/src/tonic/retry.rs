use super::BoxBody;
use http::{HeaderValue, Request, Response};
use qcs_api_client_common::{
    backoff::{self, backoff::Backoff, ExponentialBackoff},
    configuration::TokenError,
};
use tonic::{client::GrpcService, Status};

use qcs_api_client_common::backoff::duration_from_response as duration_from_http_response;
use std::{
    future::{poll_fn, Future},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use super::{build_duplicate_request, RequestBodyDuplicationError};
use tower::Layer;

/// The [`Layer`] used to apply exponential backoff retry logic to requests.
#[derive(Debug, Clone)]
pub struct RetryLayer {
    pub(crate) backoff: ExponentialBackoff,
}

impl Default for RetryLayer {
    fn default() -> Self {
        Self {
            backoff: backoff::default_backoff(),
        }
    }
}

impl<S: GrpcService<BoxBody>> Layer<S> for RetryLayer {
    type Service = RetryService<S>;

    fn layer(&self, service: S) -> Self::Service {
        Self::Service {
            backoff: self.backoff.clone(),
            service,
        }
    }
}

/// The [`GrpcService`] that wraps the gRPC client in order to provide exponential backoff retry
/// logic.
///
/// This middleware will add a `x-request-id` header to each request with a unique UUID and a
/// `x-request-retry-index` header with the number of retries that have been attempted for the
/// request.
///
/// See also: [`RetryLayer`].
#[derive(Clone, Debug)]
pub struct RetryService<S: GrpcService<BoxBody>> {
    backoff: ExponentialBackoff,
    service: S,
}

/// Return `Some` if the request should be retried and the provided `backoff`
/// has another backoff to try, or, for an http request, if the response
/// specifies a `Retry-After` header. If `None` is returned, the request should
/// not be retried.
fn duration_from_response<T>(
    response: &Response<T>,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    if let Some(grpc_status) = Status::from_header_map(response.headers()) {
        match grpc_status.code() {
            // gRPC has no equivalent to RETRY-AFTER, so just use the backoff
            tonic::Code::Unavailable => backoff.next_backoff(),
            // No other gRPC statuses are retried.
            _ => None,
        }
    } else {
        duration_from_http_response(response.status(), response.headers(), backoff)
    }
}

impl<S> GrpcService<BoxBody> for RetryService<S>
where
    S: GrpcService<BoxBody> + Send + Clone + 'static,
    S::Future: Send,
    S::ResponseBody: Send,
    super::error::Error<TokenError>: From<S::Error> + From<RequestBodyDuplicationError>,
{
    type ResponseBody = <S as GrpcService<BoxBody>>::ResponseBody;
    type Error = super::error::Error<TokenError>;
    type Future =
        Pin<Box<dyn Future<Output = Result<Response<Self::ResponseBody>, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service
            .poll_ready(cx)
            .map_err(super::error::Error::from)
    }

    fn call(&mut self, mut req: Request<BoxBody>) -> Self::Future {
        if let Ok(request_id) = new_request_id() {
            req.headers_mut().insert(KEY_X_REQUEST_ID, request_id);
        }

        // Clone the `backoff` so that new requests don't reuse it
        // and so that the `backoff` can be moved into the async closure.
        let mut backoff = self.backoff.clone();
        let mut service = self.service.clone();
        // It is necessary to replace self.service with the above clone
        // because the cloned version may not be "ready".
        //
        // See this github issue for more context:
        // https://github.com/tower-rs/tower/issues/547
        std::mem::swap(&mut self.service, &mut service);

        super::common::pin_future_with_otel_context_if_available(async move {
            let mut attempt = 0;
            loop {
                let (mut request, retained) = build_duplicate_request(req).await?;
                req = retained;

                // Ensure that the service is ready before trying to use it.
                // Failure to do this *will* cause a panic.
                poll_fn(|cx| -> Poll<Result<(), _>> { service.poll_ready(cx) })
                    .await
                    .map_err(super::error::Error::from)?;

                if let Ok(retry_index_header_value) =
                    http::HeaderValue::from_str(attempt.to_string().as_str())
                {
                    request
                        .headers_mut()
                        .insert(KEY_X_REQUEST_RETRY_INDEX, retry_index_header_value);
                }
                let duration = match service.call(request).await {
                    Ok(response) => {
                        if let Some(duration) = duration_from_response(&response, &mut backoff) {
                            duration
                        } else {
                            break Ok(response);
                        }
                    }
                    Err(error) => break Err(super::error::Error::from(error)),
                };

                tokio::time::sleep(duration).await;
                attempt += 1;
            }
        })
    }
}

fn new_request_id() -> Result<HeaderValue, http::header::InvalidHeaderValue> {
    let request_id = uuid::Uuid::new_v4().to_string();
    HeaderValue::from_str(request_id.as_str())
}

const KEY_X_REQUEST_ID: &str = "x-request-id";
const KEY_X_REQUEST_RETRY_INDEX: &str = "x-request-retry-index";

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::tonic::uds_grpc_stream;
    use crate::tonic::wrap_channel_with_retry;

    use super::*;
    use ::backoff::ExponentialBackoffBuilder;
    use tonic::server::NamedService;
    use tonic::Request;
    use tonic_health::pb::health_check_response::ServingStatus;
    use tonic_health::pb::health_server::{Health, HealthServer};
    use tonic_health::{pb::health_client::HealthClient, server::HealthService};

    struct FlakyHealthService {
        required_tries_count: AtomicUsize,
    }

    impl FlakyHealthService {
        const fn new(required_tries_count: usize) -> Self {
            Self {
                required_tries_count: AtomicUsize::new(required_tries_count),
            }
        }

        #[allow(clippy::result_large_err)]
        fn make_response(&self) -> Result<tonic_health::pb::HealthCheckResponse, Status> {
            let remaining = self.required_tries_count.fetch_sub(1, Ordering::SeqCst);
            if remaining == 0 {
                let response = tonic_health::pb::HealthCheckResponse {
                    status: ServingStatus::Serving as i32,
                };
                Ok(response)
            } else {
                self.required_tries_count
                    .store(remaining - 1, Ordering::SeqCst);
                Err(Status::unavailable("unavailable"))
            }
        }
    }

    impl Default for FlakyHealthService {
        fn default() -> Self {
            Self::new(3)
        }
    }

    #[tonic::async_trait]
    impl Health for FlakyHealthService {
        type WatchStream = tokio_stream::wrappers::ReceiverStream<
            Result<tonic_health::pb::HealthCheckResponse, Status>,
        >;

        async fn check(
            &self,
            _request: Request<tonic_health::pb::HealthCheckRequest>,
        ) -> Result<tonic::Response<tonic_health::pb::HealthCheckResponse>, Status> {
            self.make_response().map(tonic::Response::new)
        }

        async fn watch(
            &self,
            _request: Request<tonic_health::pb::HealthCheckRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, Status> {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tx.send(self.make_response()).await.unwrap();
            Ok(tonic::Response::new(
                tokio_stream::wrappers::ReceiverStream::new(rx),
            ))
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_retry_logic() {
        let health_server = HealthServer::new(FlakyHealthService::default());

        uds_grpc_stream::serve(health_server, |channel| async {
            let wrapped_channel = wrap_channel_with_retry(channel);
            let response = HealthClient::new(wrapped_channel)
                .check(Request::new(tonic_health::pb::HealthCheckRequest {
                    service: <HealthServer<HealthService> as NamedService>::NAME.to_string(),
                }))
                .await
                .unwrap();
            assert_eq!(response.into_inner().status(), ServingStatus::Serving);
        })
        .await
        .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_retry_is_not_infinite_long() {
        let health_server = HealthServer::new(FlakyHealthService::new(50));

        uds_grpc_stream::serve(health_server, |channel| async {
            let status = HealthClient::new(
                RetryLayer {
                    backoff: ExponentialBackoffBuilder::new()
                        .with_max_interval(Duration::from_millis(100))
                        .with_max_elapsed_time(Some(Duration::from_secs(1)))
                        .build(),
                }
                .layer(channel),
            )
            .check(Request::new(tonic_health::pb::HealthCheckRequest {
                service: <HealthServer<HealthService> as NamedService>::NAME.to_string(),
            }))
            .await
            .unwrap_err();
            assert_eq!(status.code(), tonic::Code::Unavailable);
        })
        .await
        .unwrap();
    }
}
