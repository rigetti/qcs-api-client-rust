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

//! This module contains common utilities for the tonic client.

/// The HTTP2 header to check for the gRPC status code.
/// See <https://github.com/grpc/grpc/blob/master/doc/PROTOCOL-HTTP2.md#responses>.
const GRPC_STATUS_CODE_HEADER_NAME: &str = "grpc-status";

#[derive(Debug)]
pub(crate) enum ParsedStatusCodeError {
    StatusHeaderMissing,
    HeaderNotString,
    HeaderNotInt,
}

/// Extract the gRPC status code from the headers of a response.
///
/// # Errors
///
/// See [`ParsedStatusCodeError`] for the possible error variants.
pub(crate) fn get_status_code_from_headers(
    header_map: &http::header::HeaderMap,
) -> Result<tonic::Code, ParsedStatusCodeError> {
    header_map
        .get(GRPC_STATUS_CODE_HEADER_NAME)
        .ok_or(ParsedStatusCodeError::StatusHeaderMissing)
        .and_then(|status| {
            status
                .to_str()
                .map_err(|_| ParsedStatusCodeError::HeaderNotString)
        })
        .and_then(|status| {
            status
                .parse::<i32>()
                .map_err(|_| ParsedStatusCodeError::HeaderNotInt)
        })
        .map(tonic::Code::from)
}

/// Wrap the future in [`opentelemetry::trace::WithContext`] if the "tracing-opentelemetry" feature
/// is enabled. This ensures the OpenTelemetry context is propagated across async boundaries. The
/// result is then pinned and boxed.
///
/// See <https://docs.rs/opentelemetry/latest/opentelemetry/trace/trait.FutureExt.html> for more
/// information.
///
/// Note, this function is intended to draw attention to the common pattern of wrapping
/// futures within [`Box::pin`] when [`tonic::client::GrpcService::call`] requires
/// asynchonous processing and we want the OpenTelemetry context propagated.
#[cfg(feature = "tracing-opentelemetry")]
pub(super) fn pin_future_with_otel_context_if_available<F>(
    fut: F,
) -> std::pin::Pin<Box<opentelemetry::trace::WithContext<F>>> {
    Box::pin(opentelemetry::trace::FutureExt::with_current_context(fut))
}

/// Pin and box a future, as is common when [`tonic::client::GrpcService::call`] requires
/// asynchronous processing.
///
/// This trivial implementation is used to distinguish between the implementation
/// with and without the "tracing-opentelemetry" feature.
#[cfg(not(feature = "tracing-opentelemetry"))]
pub(super) fn pin_future_with_otel_context_if_available<F>(fut: F) -> std::pin::Pin<Box<F>> {
    Box::pin(fut)
}
