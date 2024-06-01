//! Exponential backoff for use with QCS.
//!
//! This re-exports types from [`backoff`](::backoff) and provides a [`default_backoff`] function
//! to create a more useful default [`ExpontentialBackoff`].

use std::time::Duration;

use http::StatusCode;

pub use ::backoff::*;
use ::backoff::backoff::Backoff;

/// Create a default [`ExponentialBackoff`] for use with QCS.
///
/// This backoff will retry for up to 5 minutes, with a maximum interval of 30 seconds and some
/// randomized jitter.
#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn default_backoff() -> ExponentialBackoff {
    ExponentialBackoffBuilder::new()
        .with_max_elapsed_time(Some(Duration::from_secs(300)))
        .with_max_interval(Duration::from_secs(30))
        .build()
}

/// Return `true` if the status code is one that could be retried.
#[must_use]
pub const fn status_code_is_retry(code: StatusCode) -> bool {
    matches!(
        code,
        StatusCode::SERVICE_UNAVAILABLE | StatusCode::TOO_MANY_REQUESTS
    )
}

/// Return `Some` if the response specifies a `Retry-After` header or the provided `backoff` has
/// another backoff to try.
#[must_use]
pub fn duration_from_response(
    status: StatusCode,
    headers: &http::HeaderMap,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    use time::{format_description::well_known::Rfc2822, OffsetDateTime};

    if status_code_is_retry(status) {
        if let Some(value) = headers.get(http::header::RETRY_AFTER) {
            if let Ok(value) = value.to_str() {
                if let Ok(value) = value.parse::<u64>() {
                    return Some(Duration::from_secs(value));
                } else if let Ok(date) = OffsetDateTime::parse(value, &Rfc2822) {
                    let duration = date - OffsetDateTime::now_utc();
                    // This will fail if the number is too large or negative
                    let millis = duration.whole_milliseconds().try_into().ok()?;
                    return Some(Duration::from_millis(millis));
                }
            }
        }

        backoff.next_backoff()
    } else {
        None
    }
}

fn can_retry_method(method: &http::Method) -> bool {
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
    method: &http::Method,
    error: &reqwest::Error,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    if can_retry_method(method) {
        // There is no exposed method to inspect the inner hyper error in the reqwest error, only
        // `is_*` methods. There is no reqwest method corresponding to the hyper `is_closed`, so we
        // inspect the debug string instead.
        if error.is_timeout() || error.is_connect() || error.is_request() || format!("{error:?}").contains("source: hyper::Error(ChannelClosed)") {
            backoff.next_backoff()
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
    method: &http::Method,
    error: &std::io::Error,
    backoff: &mut ExponentialBackoff,
) -> Option<Duration> {
    use std::io::ErrorKind;
    if can_retry_method(method) {
        if matches!(error.kind(), ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted) {
            backoff.next_backoff()
        } else {
            None
        }
    } else {
        None
    }
}
