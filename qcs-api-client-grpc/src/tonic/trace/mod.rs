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

//! This modules supports tracing of gRPC requests using the
//! [`tower_http::trace::TraceLayer`]. This module customizes that functionality
//! by adhering to OpenTelemetry conventions for gRPC tracing (see
//! <https://opentelemetry.io/docs/specs/semconv/rpc/grpc/>). Note this is the
//! case whether the "tracing-opentelemetry" feature is enabled or not.
//!
//! The "tracing-opentelemetry" feature extends the base "tracing" feature in
//! two ways:
//!
//! 1. It adds additional, dynamically defined span attributes to the gRPC span
//!    using [`tracing_opentelemetry::OpenTelemetrySpanExt::set_attribute`]. This
//!    is leveraged to add "rpc.grpc.{request, response}.metadata" attributes as
//!    configured by the user.
//! 2. It supports propagation of the OpenTelemetry context via
//!    [`opentelemetry_sdk::propagation::TraceContextPropagator`].
//!
//! All of this behavior is configurable via
//! [`qcs_api_client_common::tracing_configuration::TracingConfiguration`].
pub(super) mod shared;
#[cfg(not(feature = "tracing-opentelemetry"))]
mod trace_layer;
#[cfg(feature = "tracing-opentelemetry")]
mod trace_layer_otel_ext;

#[cfg(feature = "tracing-opentelemetry")]
pub(crate) use trace_layer_otel_ext::{
    build_layer as build_trace_layer, CustomTraceLayer, CustomTraceService,
};

#[cfg(not(feature = "tracing-opentelemetry"))]
pub(crate) use trace_layer::{
    build_layer as build_trace_layer, CustomTraceLayer, CustomTraceService,
};
