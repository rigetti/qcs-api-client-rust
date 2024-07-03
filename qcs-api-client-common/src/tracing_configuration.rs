// Copyright 2023 Rigetti Computing
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

//! This module contains configuration for tracing with Open Telemetry. Most notably, it
//! provides a tracing filter which enables clients to filter which spans are sent to a
//! tracing subscriber.
//!
//! # Examples
//!
//! ```
//! use qcs_api_client_common::tracing_configuration::{TracingFilter, TracingFilterBuilder};
//! use urlpattern::UrlPatternMatchInput;
//!
//! let mut filter = TracingFilter::builder()
//!     .parse_strs_and_set_paths(&[
//!         "https://host1.api.com/v1/path1",
//!         // matches on any host
//!         "/v1/path2",
//!         "tcp://host2.api.com:5555/rpc",
//!         // matches any id, as specified in https://wicg.github.io/urlpattern/
//!         "https://host1.api.com/v1/resource/:id"
//!     ])
//!     .expect("failed to parse filter paths")
//!     .build();
//! let examples = vec![
//!     ("https://host1.api.com/v1/path1", true),
//!     ("https://host3.api.com/v1/path1", false),
//!     ("https://host3.api.com/v1/path2", true),
//!     ("tcp://host2.api.com:5555/rpc", true),
//!     ("https://host1.api.com/v1/resource/any", true)
//! ];
//!
//! examples.iter().for_each(|(url, matches)| {
//!     let input = UrlPatternMatchInput::Url(url::Url::parse(url).unwrap());
//!     assert_eq!(*matches, filter.is_enabled(&input));
//! });
//!
//! // turn the inclusive filter into an exclusive filter
//! filter = TracingFilterBuilder::from(filter).set_is_negated(true).build();
//!
//! examples.iter().for_each(|(url, matches)| {
//!     let input = UrlPatternMatchInput::Url(url::Url::parse(url).unwrap());
//!     assert_ne!(*matches, filter.is_enabled(&input));
//! });
//! ```

use std::str::FromStr;

pub use urlpattern::{UrlPatternInit, UrlPatternMatchInput, UrlPatternResult};

use {std::env, thiserror::Error, urlpattern::UrlPattern};

/// Environment variable for controlling whether any network API calls are traced.
pub static QCS_API_TRACING_ENABLED: &str = "QCS_API_TRACING_ENABLED";
/// Environment variable for controlling whether network API calls set Open Telemetry
/// context propagation headers.
pub static QCS_API_PROPAGATE_OTEL_CONTEXT: &str = "QCS_API_PROPAGATE_OTEL_CONTEXT";
/// Environment variable for filtering which network API calls are traced.
pub static QCS_API_TRACING_FILTER: &str = "QCS_API_TRACING_FILTER";
/// Environment variable to turn the tracing filter into an exclusive filter.
pub static QCS_API_NEGATE_TRACING_FILTER: &str = "QCS_API_NEGATE_TRACING_FILTER";

/// An error indicating that a tracing filter URL pattern could not be parsed. Note that tracing
/// filters must conform to [URL Pattern API syntax](https://wicg.github.io/urlpattern/). Only
/// https, http, and tcp schemes are supported.
#[derive(Error, Debug)]
pub enum TracingFilterError {
    /// The pattern is not a valid URL pattern.
    #[error("invalid url `{url}`: {error}")]
    InvalidUrl {
        /// The invalid URL.
        url: String,
        /// The source parse error returned by the URL pattern parser.
        error: url::ParseError,
    },
    /// The specified URL scheme is not supported.
    #[error("trace filtering only supports https, http, and tcp urls, found: `{0}`")]
    UnsupportedUrlScheme(String),
}

/// A builder for [`TracingConfiguration`].
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct TracingConfigurationBuilder {
    filter: Option<TracingFilter>,
    propagate_otel_context: bool,
}

impl From<TracingConfiguration> for TracingConfigurationBuilder {
    fn from(tracing_configuration: TracingConfiguration) -> Self {
        Self {
            filter: tracing_configuration.filter,
            propagate_otel_context: tracing_configuration.propagate_otel_context,
        }
    }
}

impl TracingConfigurationBuilder {
    #![allow(clippy::missing_const_for_fn)]

    /// Set a [`TracingFilter`] to be set on the [`TracingConfiguration`]. When this is set to
    /// `None`, no filter is set and all network API calls are traced.
    #[must_use]
    pub fn set_filter(mut self, filter: Option<TracingFilter>) -> Self {
        self.filter = filter;
        self
    }

    /// Sets `propagate_otel_context` which indicates whether Open Telelmetry context propagation
    /// headers should be set on network API calls.
    #[must_use]
    pub fn set_propagate_otel_context(mut self, propagate_otel_context: bool) -> Self {
        self.propagate_otel_context = propagate_otel_context;
        self
    }

    /// Build a [`TracingConfiguration`] based on this builder's values.
    #[must_use]
    pub fn build(self) -> TracingConfiguration {
        TracingConfiguration {
            filter: self.filter,
            propagate_otel_context: self.propagate_otel_context,
        }
    }
}

/// Configuration for tracing of network API calls. Note, this does not configure any trace
/// processing or collector. Rather, it configures which network API calls should be traced.
#[derive(Debug, Clone, Default)]
pub struct TracingConfiguration {
    /// An optional [`TracingFilter`].
    filter: Option<TracingFilter>,
    /// Whether or not API calls should set Open Telemetry context propagation headers.
    propagate_otel_context: bool,
}

impl TracingConfiguration {
    #![allow(clippy::missing_const_for_fn)]

    /// Create a [`TracingConfigurationBuilder`] to build a new [`TracingConfiguration`].
    #[must_use]
    pub fn builder() -> TracingConfigurationBuilder {
        TracingConfigurationBuilder::default()
    }

    /// Load tracing configuration from environment variables. Will return [`TracingFilterError`] if
    /// there is an issue parsing the tracing filter. Will return `None` if tracing is not enabled.
    ///
    /// # Errors
    ///
    /// See [`TracingFilterError`].
    pub fn from_env() -> Result<Option<Self>, TracingFilterError> {
        if !is_env_var_true(QCS_API_TRACING_ENABLED) {
            return Ok(None);
        }
        let filter = TracingFilter::from_env()?;
        let propagate_otel_context = is_env_var_true(QCS_API_PROPAGATE_OTEL_CONTEXT);
        Ok(Some(Self {
            filter,
            propagate_otel_context,
        }))
    }

    /// Get the [`TracingFilter`], if any, for this configuration.
    #[must_use]
    pub fn filter(&self) -> Option<&TracingFilter> {
        self.filter.as_ref()
    }

    /// Indicates whether Open Telemetry context propagation headers should be set on
    /// API calls.
    #[must_use]
    pub fn propagate_otel_context(&self) -> bool {
        self.propagate_otel_context
    }

    /// Returns `true` if the specified URL should be traced. For details on how this is determined,
    /// see [`TracingFilter`].
    ///
    /// Defaults to `true` if no filter is set.
    #[must_use]
    pub fn is_enabled(&self, url: &UrlPatternMatchInput) -> bool {
        self.filter
            .as_ref()
            .map_or(true, |filter| filter.is_enabled(url))
    }
}

impl From<TracingFilter> for TracingFilterBuilder {
    fn from(tracing_filter: TracingFilter) -> Self {
        Self {
            paths: tracing_filter.paths,
            is_negated: tracing_filter.is_negated,
        }
    }
}

/// Builder for [`TracingFilter`] to set/override items.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Clone)]
pub struct TracingFilterBuilder {
    is_negated: bool,
    paths: Vec<UrlPatternInit>,
}

impl TracingFilterBuilder {
    #![allow(clippy::missing_const_for_fn)]

    /// Set whether the filter paths should be negated, i.e. applied as an exclusive filter rather
    /// than an inclusive one.
    #[must_use]
    pub fn set_is_negated(mut self, is_negated: bool) -> Self {
        self.is_negated = is_negated;
        self
    }

    /// Set a `Vec` of [`UrlPatternInit`]s on which to match requests.
    #[must_use]
    pub fn set_paths(mut self, paths: Vec<UrlPatternInit>) -> Self {
        self.paths = paths;
        self
    }

    /// Parse specified strings into a `Vec` of [`UrlPatternInit`]s on which to match requests.
    ///
    /// # Errors
    ///
    /// See [`TracingFilterError`].
    pub fn parse_strs_and_set_paths(self, paths: &[&str]) -> Result<Self, TracingFilterError> {
        Ok(self.set_paths(
            paths
                .iter()
                .map(|s| parse_constructor_string(s))
                .collect::<Result<Vec<UrlPatternInit>, TracingFilterError>>()?,
        ))
    }

    /// Build a [`TracingFilter`] based on this builder's values.
    #[must_use]
    pub fn build(self) -> TracingFilter {
        TracingFilter {
            is_negated: self.is_negated,
            paths: self.paths,
        }
    }
}

/// Filter which network API calls produce tracing spans.
#[derive(Debug, Clone, Default)]
pub struct TracingFilter {
    /// When true, a request matching any URL pattern in the specified paths will NOT be traced.
    /// When false, a request matching any URL pattern in the specified paths will be traced.
    is_negated: bool,
    /// A list of URL patterns which will be used to filter requests. See documentation for
    /// `is_negated` to understand how this list is used.
    paths: Vec<UrlPatternInit>,
}

impl TracingFilter {
    /// Create a [`TracingFilterBuilder`] to build a new [`TracingFilter`].
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn builder() -> TracingFilterBuilder {
        TracingFilterBuilder::default()
    }

    /// Create a new tracing filter from the environment variables `QCS_API_TRACING_FILTER` and
    /// `QCS_API_NEGATE_TRACING_FILTER`. The `QCS_API_TRACING_FILTER` variable should contain a
    /// comma-separated list of URL patterns. The `QCS_API_NEGATE_TRACING_FILTER` variable should
    /// contain a boolean value indicating whether the filter should be negated. See the
    /// documentation for [`TracingFilter`] for more information.
    ///
    /// Note, filters may be specified as full URLs (e.g. `http://api.example.com/api/v2/jobs`),
    /// which will match on the full URL of the traced
    /// request, or as URL paths (e.g. `/api/v2/jobs`), which will match only on the path of the traced
    /// request, regardless of the base url.
    ///
    /// # Errors
    ///
    /// See [`TracingFilterError`].
    pub fn from_env() -> Result<Option<Self>, TracingFilterError> {
        if let Ok(filter) = env::var(QCS_API_TRACING_FILTER) {
            let is_negated = env::var(QCS_API_NEGATE_TRACING_FILTER)
                .map_or(false, |_| is_env_var_true(QCS_API_NEGATE_TRACING_FILTER));
            return Ok(Self::builder()
                .parse_strs_and_set_paths(&filter.split(',').collect::<Vec<_>>())?
                .set_is_negated(is_negated)
                .build()
                .into());
        }
        Ok(None)
    }

    /// Returns the first match. If evaluation of any pattern results in an error, the error is
    /// logged, but otherwise ignored (i.e. prevents poison pill effects).
    fn first_match(&self, input: &UrlPatternMatchInput) -> Option<UrlPatternResult> {
        self.paths.iter().find_map(|init| {
            <UrlPattern>::parse(init.clone())
                .and_then(|pattern| pattern.exec(input.clone()))
                .map_err(|e| {
                    tracing::error!("error matching url pattern: {}", e);
                })
                .ok()
                .flatten()
        })
    }

    /// Returns `true` if the specified URL should be traced. For details on how this is determined,
    /// see [`TracingFilter`].
    ///
    /// Note, if any regular expression executor for matching returns an error, this function will
    /// log the error, but continue checking other patterns.
    #[must_use]
    pub fn is_enabled(&self, input: &UrlPatternMatchInput) -> bool {
        let first_match = self.first_match(input);
        if self.is_negated {
            first_match.is_none()
        } else {
            first_match.is_some()
        }
    }
}

impl FromStr for TracingFilter {
    type Err = TracingFilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let paths: Vec<UrlPatternInit> = s
            .split(',')
            .map(parse_constructor_string)
            .collect::<Result<Vec<UrlPatternInit>, TracingFilterError>>()?;
        Ok(Self {
            is_negated: false,
            paths,
        })
    }
}

/// Parse a string into a [`UrlPatternInit`]. If parsing the filter string fails, we re-attempt
/// using a known valid base URL. If the second attempt succeeds, the [`UrlPatternInit`] will
/// match any URL with a matching path, regardless of its base URL.
fn parse_constructor_string(filter: &str) -> Result<UrlPatternInit, TracingFilterError> {
    url::Url::options()
        .parse(filter)
        .map_err(|error| TracingFilterError::InvalidUrl {
            url: filter.to_string(),
            error,
        })
        .and_then(validate_url_scheme)
        .and_then(|fully_specified_url| {
            url_origin_to_url(&fully_specified_url)
                .map(|base_url| url_to_url_pattern_init(&fully_specified_url, Some(base_url)))
        })
        .or_else(|original_error| {
            let baseless_url_pattern_init = url::Url::options()
                .base_url(Some(
                    // This base URL will not be included in the final [`UrlPatternInit`]; it
                    // is used to bootstrap a valid URL in the case the client specified a path
                    // rather than a full URL.
                    &url::Url::parse("https://api.qcs.rigetti.com")
                        .expect("base url bootstrap value should always parse"),
                ))
                .parse(filter)
                .map(|url_with_bootstrapped_base_url| {
                    // By passing None, we indicate that the base URL should not be included in the
                    // [`UrlPatternInit`].
                    url_to_url_pattern_init(&url_with_bootstrapped_base_url, None)
                });

            baseless_url_pattern_init.map_err(|_| original_error)
        })
}

fn url_origin_to_url(value: &url::Url) -> Result<url::Url, TracingFilterError> {
    value
        .origin()
        .unicode_serialization()
        .parse()
        .map_err(|error| TracingFilterError::InvalidUrl {
            url: value.to_string(),
            error,
        })
}

fn url_to_url_pattern_init(value: &url::Url, base_url: Option<url::Url>) -> UrlPatternInit {
    UrlPatternInit {
        // these values come from the base url. If base url is None, these are left unspecified.
        protocol: base_url.as_ref().map(|v| v.scheme().to_string()),
        username: base_url
            .as_ref()
            .map(|v| v.username().to_string())
            .filter(|s| !s.is_empty()),
        password: base_url
            .as_ref()
            .and_then(|v| v.password().map(String::from)),
        hostname: base_url
            .as_ref()
            .and_then(|v| v.host_str().map(String::from)),
        port: base_url
            .as_ref()
            .and_then(|v| v.port().map(|p| p.to_string())),

        // these values come from the url literal specified in the TracingFilter constructor
        pathname: Some(value.path().to_string()).filter(|s| !s.is_empty()),
        search: value.query().map(String::from),
        hash: value.fragment().map(String::from),
        base_url,
    }
}

fn validate_url_scheme(value: url::Url) -> Result<url::Url, TracingFilterError> {
    if let "http" | "https" | "tcp" = value.scheme() {
        Ok(value)
    } else {
        Err(TracingFilterError::UnsupportedUrlScheme(
            value.scheme().to_string(),
        ))
    }
}

fn is_env_var_true(var: &str) -> bool {
    matches!(env::var(var), Ok(e) if matches!(e.to_lowercase().as_str(), "true" | "t" | "1" | "yes" | "y" | "on"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "https://api.qcs.rigetti.com/v1/users/:id",
        "https://api.qcs.rigetti.com/v1/users/10",
        true
    )]
    #[case(
        "https://api.qcs.rigetti.com/v1/users/:id",
        "https://api.dev.qcs.rigetti.com/v1/users/10",
        false
    )]
    #[case("/v1/users/:id", "https://api.qcs.rigetti.com/v1/users/10", true)]
    #[case("/v1/users/:id", "https://api.qcs.rigetti.com/v1/groups/10", false)]
    #[case("tcp://localhost:5555", "tcp://localhost:5555", true)]
    #[case("tcp://localhost:5555/my_rpc", "tcp://localhost:5555/my_rpc", true)]
    #[case("tcp://localhost:5555/my_rpc", "tcp://localhost:5555/other_rpc", false)]
    #[case("/my_rpc", "tcp://localhost:5555/my_rpc", true)]
    fn test_tracing_filter(#[case] filter: &str, #[case] url: &str, #[case] matches: bool) {
        let input = UrlPatternMatchInput::Url(url::Url::parse(url).unwrap());

        let mut tracing_filter = TracingFilter::from_str(filter).unwrap();
        assert_eq!(tracing_filter.is_enabled(&input), matches);

        tracing_filter = TracingFilterBuilder::from(tracing_filter)
            .set_is_negated(true)
            .build();
        assert_eq!(tracing_filter.is_enabled(&input), !matches);
    }
}
