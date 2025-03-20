// Copyright 2024 Rigetti Computing
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

//! Shared utilities for tracing gRPC requests across the "tracing-opentelemetry" and
//! base "tracing" feature sets.
use qcs_api_client_common::tracing_configuration::TracingFilter;
use tower_http::classify::GrpcFailureClass;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use urlpattern::UrlPatternMatchInput;

/// Creates a gRPC request span that conforms to the gRPC semantic conventions.
/// See <https://opentelemetry.io/docs/specs/semconv/rpc/grpc/>
/// for details.
pub(super) fn make_grpc_request_span<B>(request: &http::Request<B>) -> tracing::Span {
    let url = request.uri();
    let path = url.path();
    let mut path_split = path.split('/');
    let (_, service, method) = (path_split.next(), path_split.next(), path_split.next());
    let service = service.unwrap_or("");
    let method = method.unwrap_or("");
    let host = url.host().unwrap_or("");
    let host_port = url.port().map_or(0u16, |p| p.as_u16());

    tracing::span!(
        tracing::Level::INFO,
        "gRPC request",
        rpc.system = "grpc",
        rpc.service = %service,
        rpc.method = %method,
        net.peer.name = %host,
        net.peer.port = %host_port,
        "message.type" = "sent",
        otel.kind = "client",
        otel.name = %path,
        rpc.grpc.status_code = tracing::field::Empty,
    )
}

/// Determines whether a request should be traced based on the request URL and the
/// configured tracing filter.
///
/// If `filter` is `None`, the request should be traced.
pub(super) fn should_trace_request<B>(
    base_url: &str,
    request: &http::Request<B>,
    filter: Option<&TracingFilter>,
) -> bool {
    // The request URI here doesn't include the base url, so we have  to manually add it here to evaluate request filter patterns.
    let full_request_url = format!("{base_url}{}", request.uri());

    let parsed = full_request_url.parse::<::url::Url>();
    let url = parsed.ok();
    filter
        .and_then(|filter| url.map(|url| (filter, url)))
        .map_or(true, |(filter, url)| {
            filter.is_enabled(&UrlPatternMatchInput::Url(url))
        })
}

/// A [`tower_http::trace::OnFailure`] implementation for gRPC requests.
///
/// Sets the "rpc.grpc.status_code" attribute on the span if the failure classification
/// is [`GrpcFailureClass::Code`]; otherwise, it sets the status code to
/// [`tonic::Code::Unknown`].
#[derive(Clone, Debug, Default)]
pub struct OnFailure {
    pub(super) inner: tower_http::trace::DefaultOnFailure,
}

impl tower_http::trace::OnFailure<GrpcFailureClass> for OnFailure {
    fn on_failure(
        &mut self,
        failure_classification: GrpcFailureClass,
        latency: std::time::Duration,
        span: &Span,
    ) {
        match failure_classification {
            GrpcFailureClass::Code(code) => {
                span.set_attribute("rpc.grpc.status_code", format!("{code}"));
            }
            GrpcFailureClass::Error(_) => {
                span.set_attribute(
                    "rpc.grpc.status_code",
                    format!("{}", tonic::Code::Unknown as u8),
                );
            }
        }

        self.inner.on_failure(failure_classification, latency, span);
    }
}
