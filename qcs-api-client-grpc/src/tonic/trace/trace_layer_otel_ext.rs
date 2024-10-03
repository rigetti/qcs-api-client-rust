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
//! for details on the OpenTelemetry semantic conventions for gRPC.
//!
//! It extends the set of features provided by the base "tracing" feature in
//! three ways:
//!
//! 1. The [`tower_http::trace::MakeSpan`] implementation creates a span with
//!    the current [`opentelemetry::Context`] as the parent.
//! 2. It supports propagation of the OpenTelemetry context to the server via
//!    [`opentelemetry_sdk::propagation::TraceContextPropagator`].
//! 3. It leverages the interior mutability supported by
//!    [`tracing_opentelemetry::OpenTelemetrySpanExt::set_attribute`] to add
//!    "rpc.grpc.{request, response}.metadata" attributes.
//!
//! All of this behavior is configurable via
//! [`qcs_api_client_common::tracing_configuration::TracingConfiguration`].
use crate::tonic::common::get_status_code_from_headers;
use http::{HeaderMap, HeaderValue};
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::trace::FutureExt;
use opentelemetry::trace::WithContext;
use opentelemetry_http::HeaderInjector;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use qcs_api_client_common::tracing_configuration::HeaderAttributesFilter;
use qcs_api_client_common::tracing_configuration::{
    IncludeExclude, TracingConfiguration, TracingFilter,
};
use tonic::{body::BoxBody, client::GrpcService};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use super::shared::make_grpc_request_span;
use super::shared::should_trace_request;

#[derive(Debug, Clone, Copy)]
enum MetadataAttributeType {
    Request,
    Response,
}

impl std::fmt::Display for MetadataAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request => write!(f, "request"),
            Self::Response => write!(f, "response"),
        }
    }
}

/// Call [`tracing_opentelemetry::OpenTelemetrySpanExt::set_attribute`] on the specified
/// [`tracing::Span`] for each span attribute produced by
/// [`IncludeExclude::get_header_attributes`]. The attributes are formatted according to
/// the OpenTelemetry semantic conventions for gRPC.
fn set_metadata_attribute(
    span: &tracing::Span,
    include_exclude: &IncludeExclude<String>,
    headers: &HeaderMap<HeaderValue>,
    metadata_attribute_type: MetadataAttributeType,
) {
    let headers_to_trace = include_exclude.get_header_attributes(headers);
    for (key, value) in headers_to_trace.into_iter() {
        span.set_attribute(
            format!("rpc.grpc.{metadata_attribute_type}.metadata.{key}"),
            value,
        );
    }
}

/// A [`tower_http::trace::MakeSpan`] implementation for gRPC requests.
///
/// This will set any "rpc.grpc.request.metadata" attributes configured by the user.
/// It will also call [`tracing_opentelemetry::span_ext::OpenTelemetrySpanExt::set_parent`]
/// with the current [`opentelemetry::Context`].
#[derive(Clone, Debug)]
pub struct MakeSpan {
    enabled: bool,
    request_headers: IncludeExclude<String>,
    filter: Option<TracingFilter>,
    base_url: String,
}

impl<B> tower_http::trace::MakeSpan<B> for MakeSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> tracing::Span {
        if self.enabled
            && should_trace_request(self.base_url.as_str(), request, self.filter.as_ref())
        {
            let span = make_grpc_request_span(request);

            span.set_parent(opentelemetry::Context::current());
            set_metadata_attribute(
                &span,
                &self.request_headers,
                request.headers(),
                MetadataAttributeType::Request,
            );
            span
        } else {
            tracing::Span::none()
        }
    }
}

/// A [`tower_http::trace::OnEos`] implementation for gRPC requests.
///
/// This will set the "rpc.grpc.status_code" and "rpc.grpc.response.metadata" attributes
/// configured by the user if trailers are present.
#[derive(Clone, Debug)]
pub struct OnEos {
    response_headers: IncludeExclude<String>,
    inner: tower_http::trace::DefaultOnEos,
}

impl tower_http::trace::OnEos for OnEos {
    fn on_eos(
        self,
        trailers: Option<&HeaderMap>,
        stream_duration: std::time::Duration,
        span: &Span,
    ) {
        use tracing_opentelemetry::OpenTelemetrySpanExt;

        if let Some(trailers) = trailers {
            if let Ok(status_code) = get_status_code_from_headers(trailers) {
                span.set_attribute("rpc.grpc.status_code", format!("{}", status_code as u8));
            }
            set_metadata_attribute(
                span,
                &self.response_headers,
                trailers,
                MetadataAttributeType::Response,
            );
        }
        self.inner.on_eos(trailers, stream_duration, span);
    }
}

/// A [`tower_http::trace::OnResponse`] implementation for gRPC requests.
///
/// This will set any "rpc.grpc.response.metadata" attributes configured by the user.
#[derive(Clone, Debug)]
pub struct OnResponse {
    response_headers: IncludeExclude<String>,
    inner: tower_http::trace::DefaultOnResponse,
}

impl Default for OnResponse {
    fn default() -> Self {
        Self {
            response_headers: IncludeExclude::include_none(),
            inner: tower_http::trace::DefaultOnResponse::default(),
        }
    }
}

impl<B> tower_http::trace::OnResponse<B> for OnResponse {
    fn on_response(self, response: &http::Response<B>, latency: std::time::Duration, span: &Span) {
        set_metadata_attribute(
            span,
            &self.response_headers,
            response.headers(),
            MetadataAttributeType::Response,
        );
        self.inner.on_response(response, latency, span);
    }
}

type BaseTraceLayer = tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::GrpcErrorsAsFailures>,
    MakeSpan,
    tower_http::trace::DefaultOnRequest,
    OnResponse,
    tower_http::trace::DefaultOnBodyChunk,
    OnEos,
    super::shared::OnFailure,
>;

type BaseTraceService = tower_http::trace::Trace<
    tonic::transport::Channel,
    tower_http::classify::SharedClassifier<tower_http::classify::GrpcErrorsAsFailures>,
    MakeSpan,
    tower_http::trace::DefaultOnRequest,
    OnResponse,
    tower_http::trace::DefaultOnBodyChunk,
    OnEos,
    super::shared::OnFailure,
>;

/// An implementation of [`GrpcService`] that propagates the OpenTelemetry context
/// via the [`TraceContextPropagator`]. It additionally extends the base
/// [`tower_http::trace::Trace`] implementation to include gRPC span attributes.
pub struct CustomTraceService {
    propagate_trace_id: bool,
    filter: Option<TracingFilter>,
    base_url: String,
    inner: BaseTraceService,
}

impl CustomTraceService {
    /// Creates a new [`CustomTraceService`].
    ///
    /// # Arguments
    ///
    /// * `propagate_trace_id` - Whether to propagate the OpenTelemetry context.
    /// * `base_url` - The base URL of the gRPC service. This is used for matching
    ///    against the configured `TracingFilter`.
    /// * `filter` - A filter to determine which requests should be traced. If `None`,
    ///    all requests will be traced.
    /// * `inner` - The base trace service.
    pub fn new(
        propagate_trace_id: bool,
        base_url: String,
        filter: Option<TracingFilter>,
        inner: BaseTraceService,
    ) -> Self {
        Self {
            propagate_trace_id,
            filter,
            base_url,
            inner,
        }
    }
}

impl GrpcService<BoxBody> for CustomTraceService {
    type ResponseBody = <BaseTraceService as GrpcService<BoxBody>>::ResponseBody;
    type Error = <BaseTraceService as GrpcService<BoxBody>>::Error;
    type Future = WithContext<<BaseTraceService as GrpcService<BoxBody>>::Future>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        GrpcService::poll_ready(&mut self.inner, cx)
    }

    fn call(&mut self, mut request: http::Request<BoxBody>) -> Self::Future {
        if self.propagate_trace_id
            && should_trace_request(self.base_url.as_str(), &request, self.filter.as_ref())
        {
            let propagator = TraceContextPropagator::new();
            let mut injector = HeaderInjector(request.headers_mut());
            propagator.inject_context(&opentelemetry::Context::current(), &mut injector);
        }

        self.inner.call(request).with_current_context()
    }
}

/// A [`tower::Layer`] implementation for gRPC requests with OpenTelemetry attributes
/// and propagate the OpenTelemetry context if configured by the user.
#[derive(Debug, Clone)]
pub struct CustomTraceLayer {
    propagate_trace_id: bool,
    filter: Option<TracingFilter>,
    pub(super) base_url: String,
    base_trace_layer: BaseTraceLayer,
}

impl CustomTraceLayer {
    /// Creates a new [`CustomTraceLayer`].
    ///
    /// # Arguments
    ///
    /// * `propagate_trace_id` - Whether to propagate the OpenTelemetry context.
    /// * `base_url` - The base URL of the gRPC service. This is used for matching
    ///   against the configured `TracingFilter`.
    /// * `filter` - A filter to determine which requests should be traced. If `None`,
    ///   all requests will be traced.
    /// * `base_trace_layer` - The base trace layer.
    pub fn new(
        propagate_trace_id: bool,
        base_url: String,
        filter: Option<TracingFilter>,
        base_trace_layer: BaseTraceLayer,
    ) -> Self {
        Self {
            propagate_trace_id,
            filter,
            base_url,
            base_trace_layer,
        }
    }
}

impl tower::Layer<tonic::transport::Channel> for CustomTraceLayer {
    type Service = CustomTraceService;

    fn layer(&self, inner: tonic::transport::Channel) -> Self::Service {
        let traced_channel = self.base_trace_layer.layer(inner);
        CustomTraceService::new(
            self.propagate_trace_id,
            self.base_url.clone(),
            self.filter.clone(),
            traced_channel,
        )
    }
}

#[must_use]
fn build_base_trace_layer(
    base_url: String,
    configuration: Option<&TracingConfiguration>,
) -> BaseTraceLayer {
    tower_http::trace::TraceLayer::new_for_grpc()
        .on_eos(OnEos {
            inner: tower_http::trace::DefaultOnEos::default(),
            response_headers: configuration
                .as_ref()
                .map(|configuration| configuration.response_headers().clone())
                .unwrap_or_else(IncludeExclude::include_none),
        })
        .make_span_with(MakeSpan {
            enabled: configuration.is_some(),
            request_headers: configuration
                .as_ref()
                .map(|configuration| configuration.request_headers().clone())
                .unwrap_or_else(IncludeExclude::include_none),
            filter: configuration
                .as_ref()
                .and_then(|configuration| configuration.filter())
                .cloned(),
            base_url: base_url.clone(),
        })
        .on_failure(super::shared::OnFailure {
            inner: tower_http::trace::DefaultOnFailure::default(),
        })
        .on_response(OnResponse {
            inner: tower_http::trace::DefaultOnResponse::default(),
            response_headers: configuration
                .as_ref()
                .map(|configuration| configuration.response_headers().clone())
                .unwrap_or_else(IncludeExclude::include_none),
        })
}

/// Builds a base trace layer for gRPC requests with OpenTelemetry attributes,
/// including attributes dynamically configured in [`TracingConfiguration`].
///
/// # Arguments
///
/// * `base_url` - The base URL of the gRPC service. This is used for matching
///  against the configured [`TracingFilter`].
/// * `configuration` - The tracing configuration. If `None`, no requests will
///  be traced.
#[must_use]
pub fn build_layer(
    base_url: String,
    configuration: Option<&TracingConfiguration>,
) -> CustomTraceLayer {
    let trace_layer = build_base_trace_layer(base_url.clone(), configuration);

    CustomTraceLayer::new(
        configuration
            .as_ref()
            .map(|configuration| configuration.propagate_otel_context())
            .unwrap_or(false),
        base_url,
        configuration
            .as_ref()
            .and_then(|configuration| configuration.filter())
            .cloned(),
        trace_layer,
    )
}
