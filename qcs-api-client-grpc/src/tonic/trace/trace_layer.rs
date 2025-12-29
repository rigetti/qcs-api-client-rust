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

//! Provides implementations of [`tower_http::trace`] traits for gRPC requests with
//! OpenTelemetry attributes. See <https://opentelemetry.io/docs/specs/semconv/rpc/grpc/>
//! for details on the OpenTelemetry semantic conventions for gRPC. It does so to the
//! extent possible without requiring the "tracing-opentelemetry" feature dependencies.
//!
//! All of this behavior is configurable via
//! [`qcs_api_client_common::tracing_configuration::TracingConfiguration`].
//!
//! The "tracing-opentelemetry" feature extends the set of conventions supported.
//! See [`super::trace_layer_otel_ext`] module documentation for more details.
use super::shared::{make_grpc_request_span, should_trace_request};
use crate::tonic::common::get_status_code_from_headers;
use http::HeaderMap;
use qcs_api_client_common::tracing_configuration::{TracingConfiguration, TracingFilter};
use tracing::Span;

/// A [`tower_http::trace::MakeSpan`] implementation for gRPC requests.
#[derive(Clone, Debug)]
pub struct MakeSpan {
    enabled: bool,
    filter: Option<TracingFilter>,
    base_url: String,
}

impl<B> tower_http::trace::MakeSpan<B> for MakeSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        if self.enabled
            && should_trace_request(self.base_url.as_str(), request, self.filter.as_ref())
        {
            make_grpc_request_span(request)
        } else {
            tracing::Span::none()
        }
    }
}

/// A [`tower_http::trace::OnEos`] implementation for gRPC requests. This will set
/// the "rpc.grpc.status_code" attribute on the span if the status code is present
/// in the trailers.
#[derive(Clone, Debug)]
pub struct OnEos {
    inner: tower_http::trace::DefaultOnEos,
}

impl tower_http::trace::OnEos for OnEos {
    fn on_eos(
        self,
        trailers: Option<&HeaderMap>,
        stream_duration: std::time::Duration,
        span: &Span,
    ) {
        if let Some(trailers) = trailers {
            if let Ok(status_code) = get_status_code_from_headers(trailers) {
                span.record("rpc.grpc.status_code", format!("{}", status_code as u8));
            }
        }
        self.inner.on_eos(trailers, stream_duration, span);
    }
}

/// A [`tower_http::trace::TraceLayer`] implementation for gRPC requests. This customization
/// of this layer ensures that the span has the "rpc.grpc.status_code" attribute set.
pub type CustomTraceLayer = tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::GrpcErrorsAsFailures>,
    MakeSpan,
    tower_http::trace::DefaultOnRequest,
    tower_http::trace::DefaultOnResponse,
    tower_http::trace::DefaultOnBodyChunk,
    OnEos,
    super::shared::OnFailure,
>;

/// A [`tower_http::trace::Trace`] implementation for gRPC requests. This customization
/// of this service ensures that the span has the "rpc.grpc.status_code" attribute set.
pub type CustomTraceService = tower_http::trace::Trace<
    tonic::transport::Channel,
    tower_http::classify::SharedClassifier<tower_http::classify::GrpcErrorsAsFailures>,
    MakeSpan,
    tower_http::trace::DefaultOnRequest,
    tower_http::trace::DefaultOnResponse,
    tower_http::trace::DefaultOnBodyChunk,
    OnEos,
    super::shared::OnFailure,
>;

/// Builds a trace layer for gRPC requests with basic OpenTelemetry attributes.
///
/// # Arguments
///
/// * `base_url` - The base URL of the gRPC service. This is used for matching against the configured [`TracingFilter`].
/// * `configuration` - The tracing configuration. If `None`, no requests will be traced.
#[must_use]
pub fn build_layer(
    base_url: String,
    configuration: Option<&TracingConfiguration>,
) -> CustomTraceLayer {
    tower_http::trace::TraceLayer::new_for_grpc()
        .on_eos(OnEos {
            inner: tower_http::trace::DefaultOnEos::default(),
        })
        .make_span_with(MakeSpan {
            enabled: configuration.is_some(),
            filter: configuration
                .as_ref()
                .and_then(|configuration| configuration.filter())
                .cloned(),
            base_url: base_url.clone(),
        })
        .on_failure(super::shared::OnFailure {
            inner: tower_http::trace::DefaultOnFailure::default(),
        })
        .on_response(tower_http::trace::DefaultOnResponse::default())
}
