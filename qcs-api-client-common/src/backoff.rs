//! Exponential backoff for use with QCS.
//!
//! This re-exports types from [`backon`](::backon) and provides a [`default_backoff`] function
//! to create a more useful default [`ExponentialBuilder`].
//!
//! [`ExponentialBuilder`] is cheaply `Clone`/`Copy`, so it can be stored and reused to
//! [`BackoffBuilder::build`] a fresh [`ExponentialBackoff`] iterator for each retry sequence.

use std::{error::Error as _, time::Duration};

use qcs_dependencies_client::http::StatusCode;

pub use ::backon::*;

/// Create a default [`ExponentialBuilder`] for use with QCS.
///
/// The built backoff will retry for up to 5 minutes, with a maximum interval of 30 seconds and
/// some randomized jitter.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn default_backoff() -> ExponentialBuilder {
    ExponentialBuilder::new()
        .with_jitter()
        .with_min_delay(Duration::from_millis(500))
        .with_factor(1.5)
        .with_max_delay(Duration::from_secs(30))
        .with_total_delay(Some(Duration::from_secs(300)))
        .without_max_times()
}

/// Return `true` if the status code is one that could be retried.
#[must_use]
pub const fn status_code_is_retry(code: StatusCode) -> bool {
    matches!(
        code,
        StatusCode::SERVICE_UNAVAILABLE | StatusCode::BAD_GATEWAY | StatusCode::TOO_MANY_REQUESTS
    )
}

/// Return `Some` if the response specifies a `Retry-After` header or the provided `backoff` has
/// another backoff to try. If `None` is returned, the request should not be retried.
#[must_use]
pub fn duration_from_response(
    status: StatusCode,
    headers: &qcs_dependencies_client::http::HeaderMap,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    use time::{OffsetDateTime, format_description::well_known::Rfc2822};

    if status_code_is_retry(status) {
        if let Some(value) = headers.get(qcs_dependencies_client::http::header::RETRY_AFTER) {
            if let Ok(value) = value.to_str() {
                if let Ok(value) = value.parse::<u64>() {
                    return Some(Duration::from_secs(value));
                } else if let Ok(date) = OffsetDateTime::parse(value, &Rfc2822) {
                    let duration = date - OffsetDateTime::now_utc();
                    // Convert from time::Duration to std::time::Duration
                    // This will fail if the number is too large or negative
                    let std_duration: Duration = duration.try_into().ok()?;
                    return Some(std_duration);
                }
            }
        }

        backoff.next()
    } else {
        None
    }
}

fn can_retry_method(method: &qcs_dependencies_client::http::Method) -> bool {
    // Safe means the method is essentially read-only (see https://datatracker.ietf.org/doc/html/rfc7231#section-4.2.1)
    // Idempotent means multiple identical requests have the same side-effects as a single one (see https://datatracker.ietf.org/doc/html/rfc7231#section-4.2.2)

    // Idempotent methods are defined as safe methods + PUT and DELETE.
    // Since we have some API endpoints using PUT and DELETE that are not idempotent, this function
    // currently returns just safe methods.

    method.is_safe()
}

/// Return `Some` if the error is one that makes sense to retry and `method` is one that indicates
/// it is safe to retry.
#[must_use]
pub fn duration_from_reqwest_error(
    method: &qcs_dependencies_client::http::Method,
    error: &qcs_dependencies_client::reqwest::Error,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    if can_retry_method(method) {
        if error.is_timeout()
            || error.is_connect()
            || error.is_request()
            || error
                .source()
                .and_then(|inner| inner.downcast_ref::<hyper::Error>())
                .is_some_and(hyper::Error::is_closed)
        {
            backoff.next()
        } else {
            None
        }
    } else {
        None
    }
}

/// Return `Some` if the error is one that makes sense to retry and `method` is one that indicates
/// it is safe to retry.
#[must_use]
pub fn duration_from_io_error(
    method: &qcs_dependencies_client::http::Method,
    error: &std::io::Error,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    use std::io::ErrorKind;
    if can_retry_method(method) {
        if matches!(
            error.kind(),
            ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted
        ) {
            backoff.next()
        } else {
            None
        }
    } else {
        None
    }
}
