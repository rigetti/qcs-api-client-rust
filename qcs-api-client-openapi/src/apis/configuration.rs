// Copyright 2022 Rigetti Computing
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

/*
 * Rigetti QCS API
 *
 * # Introduction  This is the documentation for the Rigetti QCS HTTP API.  You can find out more about Rigetti at [https://rigetti.com](https://rigetti.com), and also interact with QCS via the web at [https://qcs.rigetti.com](https://qcs.rigetti.com).  This API is documented in **OpenAPI format** and so is compatible with the dozens of language-specific client generators available [here](https://github.com/OpenAPITools/openapi-generator) and elsewhere on the web.  # Principles  This API follows REST design principles where appropriate, and otherwise an HTTP RPC paradigm. We adhere to the Google [API Improvement Proposals](https://google.aip.dev/general) where reasonable to provide a consistent, intuitive developer experience. HTTP response codes match their specifications, and error messages fit a common format.  # Authentication  All access to the QCS API requires OAuth2 authentication provided by Okta. You can request access [here](https://www.rigetti.com/get-quantum). Once you have a user account, you can download your access token from QCS [here](https://qcs.rigetti.com/auth/token).   That access token is valid for 24 hours after issuance. The value of `access_token` within the JSON file is the token used for authentication (don't use the entire JSON file).  Authenticate requests using the `Authorization` header and a `Bearer` prefix:  ``` curl --header \"Authorization: Bearer eyJraW...Iow\" ```  # Quantum Processor Access  Access to the quantum processors themselves is not yet provided directly by this HTTP API, but is instead performed over ZeroMQ/[rpcq](https://github.com/rigetti/rpcq). Until that changes, we suggest using [pyquil](https://github.com/rigetti/pyquil) to build and execute quantum programs via the Legacy API.  # Legacy API  Our legacy HTTP API remains accessible at https://forest-server.qcs.rigetti.com, and it shares a source of truth with this API's services. You can use either service with the same user account and means of authentication. We strongly recommend using the API documented here, as the legacy API is on the path to deprecation.
 *
 * The version of the OpenAPI document: 2020-07-31
 * Contact: support@rigetti.com
 * Generated by: https://openapi-generator.tech
 */

use qcs_api_client_common::backoff;
use reqwest;
#[cfg(feature = "otel-tracing")]
use {
    qcs_api_client_common::tracing_configuration::HeaderAttributesFilter,
    reqwest_middleware::ClientBuilder, reqwest_tracing::reqwest_otel_span,
    reqwest_tracing::TracingMiddleware, tracing, tracing::Span,
};

#[derive(Debug, Clone)]
pub struct Configuration {
    #[cfg(not(feature = "otel-tracing"))]
    pub client: reqwest::Client,
    #[cfg(feature = "otel-tracing")]
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub qcs_config: crate::common::ClientConfiguration,
    pub backoff: backoff::ExponentialBackoff,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

static USER_AGENT: &str = "QCS OpenAPI Client (Rust)/2020-07-31";

impl Configuration {
    pub async fn new() -> Result<Self, crate::common::configuration::LoadError> {
        crate::common::ClientConfiguration::load_default().map(Self::with_qcs_config)
    }

    pub fn with_qcs_config(qcs_config: crate::common::ClientConfiguration) -> Configuration {
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("failed to add User-Agent to HTTP client");

        Self::with_client_and_qcs_config(client, qcs_config)
    }

    pub fn with_client_and_qcs_config(
        client: reqwest::Client,
        qcs_config: crate::common::ClientConfiguration,
    ) -> Self {
        #[cfg(feature = "otel-tracing")]
        let client = {
            use reqwest_middleware::Extension;

            let mut client_builder = ClientBuilder::new(client);
            if let Some(tracing_configuration) = qcs_config.tracing_configuration() {
                client_builder = client_builder.with_init(Extension(tracing_configuration.clone()));
                let middleware = TracingMiddleware::<FilteredSpanBackend>::new();
                client_builder = client_builder.with(middleware);
            }
            client_builder.build()
        };

        Self {
            qcs_config,
            client,
            backoff: backoff::default_backoff(),
        }
    }
}

#[cfg(feature = "otel-tracing")]
struct FilteredSpanBackend;

#[cfg(feature = "otel-tracing")]
#[derive(Debug, Clone, Copy)]
enum MetadataAttributeType {
    Request,
    Response,
}

#[cfg(feature = "otel-tracing")]
impl std::fmt::Display for MetadataAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request => write!(f, "request"),
            Self::Response => write!(f, "response"),
        }
    }
}

#[cfg(feature = "otel-tracing")]
impl FilteredSpanBackend {
    fn is_enabled(req: &reqwest::Request, extensions: &mut http::Extensions) -> bool {
        if let Some(filter) = extensions
            .get::<qcs_api_client_common::tracing_configuration::TracingConfiguration>()
            .and_then(|tracing_configuration| tracing_configuration.filter())
        {
            let input = urlpattern::UrlPatternMatchInput::Url(req.url().clone());
            return filter.is_enabled(&input);
        }
        true
    }

    fn add_header_metadata(
        span: &Span,
        header_map: &http::HeaderMap,
        extensions: &http::Extensions,
        metadata_attribute_type: MetadataAttributeType,
    ) {
        if let Some(tracing_configuration) =
            extensions.get::<qcs_api_client_common::tracing_configuration::TracingConfiguration>()
        {
            let request_headers_to_trace = tracing_configuration
                .request_headers()
                .get_header_attributes(header_map);
            for (key, value) in request_headers_to_trace {
                tracing_opentelemetry::OpenTelemetrySpanExt::set_attribute(
                    span,
                    format!("http.{metadata_attribute_type}.header.{key}"),
                    value,
                );
            }
        }
    }
}

#[cfg(feature = "otel-tracing")]
impl reqwest_tracing::ReqwestOtelSpanBackend for FilteredSpanBackend {
    /// Checks the filter to verify whether an HTTP request should be traced and produces a span for the given
    /// request that conforms to OpenTelemetry semantic conventions if so. See
    /// <https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/trace/semantic_conventions/http.md#http-client>
    /// for details about the related semantic conventions.
    fn on_request_start(
        req: &reqwest::Request,
        extensions: &mut http::Extensions,
    ) -> tracing::Span {
        if !Self::is_enabled(req, extensions) {
            return tracing::Span::none();
        }
        let uri = req.url().to_string();
        let http_target = req.url().path();
        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or("");
        let span = reqwest_otel_span!(
            name = "HTTP request",
            req,
            http.url = uri,
            http.target = http_target,
            http.user_agent = user_agent
        );
        Self::add_header_metadata(
            &span,
            req.headers(),
            extensions,
            MetadataAttributeType::Request,
        );

        span
    }

    fn on_request_end(
        span: &tracing::Span,
        outcome: &reqwest_middleware::Result<reqwest::Response>,
        extension: &mut http::Extensions,
    ) {
        if let Ok(response) = outcome {
            Self::add_header_metadata(
                span,
                response.headers(),
                extension,
                MetadataAttributeType::Response,
            );
        }

        reqwest_tracing::default_on_request_end(span, outcome)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "otel-tracing")]
    use rstest::rstest;

    /// https://docs.rs/reqwest_mock doesn't seem well maintained and requires setting the
    /// Configuration::client field to be a trait or struct from the reqwest_mock crate.
    ///
    /// Additionally, reqwest still doesn't support Unix domain sockets, so unit testing is fairly
    /// limited for here. See more info on UDS, see <https://github.com/seanmonstar/reqwest/issues/39>.

    /// Test that all requests are traced when no filter is specified.
    #[cfg(feature = "otel-tracing")]
    #[rstest]
    fn test_tracing_enabled_no_filter() {
        use crate::apis::configuration::FilteredSpanBackend;

        let request = reqwest::Request::new(
            reqwest::Method::GET,
            "https://api.qcs.rigetti.com"
                .parse()
                .expect("test url should be valid"),
        );
        let mut extensions = http::Extensions::new();
        assert!(FilteredSpanBackend::is_enabled(&request, &mut extensions));
    }

    /// Test that requests are traced according to filter patterns.
    #[cfg(feature = "otel-tracing")]
    #[rstest]
    // TODO #111: fix this test
    #[ignore]
    #[case("https://api.qcs.rigetti.com/v1/path", true)]
    #[ignore]
    #[case("https://api.qcs.rigetti.com/v1/other", false)]
    #[ignore]
    #[case("https://other.qcs.rigetti.com/v1/path", false)]
    fn test_tracing_enabled_filter_not_passed(#[case] url: &str, #[case] expected: bool) {
        use qcs_api_client_common::tracing_configuration::TracingFilterBuilder;

        use crate::apis::configuration::FilteredSpanBackend;

        let mut tracing_filter =
            qcs_api_client_common::tracing_configuration::TracingFilter::builder()
                .parse_strs_and_set_paths(&["https://api.qcs.rigetti.com/v1/path"])
                .expect("test pattern should be valid")
                .build();

        let url = url.parse().expect("test url should be valid");
        let request = reqwest::Request::new(reqwest::Method::GET, url);
        let mut extensions = http::Extensions::new();
        extensions.insert(tracing_filter.clone());
        assert_eq!(
            expected,
            FilteredSpanBackend::is_enabled(&request, &mut extensions)
        );

        tracing_filter = TracingFilterBuilder::from(tracing_filter)
            .set_is_negated(true)
            .build();
        extensions.insert(tracing_filter);
        assert_ne!(
            expected,
            FilteredSpanBackend::is_enabled(&request, &mut extensions)
        );
    }
}
