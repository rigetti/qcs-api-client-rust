//! OIDC Utilities
//!
//! This module provides utilities for OpenID Connect,
//! including the [`fetch_discovery`] function for fetching OIDC Discovery documents.

use serde::{Deserialize, Serialize};
use url::Url;

use super::error::DiscoveryError;

type Result<T> = std::result::Result<T, DiscoveryError>;

/// OIDC Discovery document.
///
/// For more information, see: <https://openid.net/specs/openid-connect-discovery-1_0.html>.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Discovery {
    /// The issuer URL from the discovery document.
    ///
    /// This is the canonical URI that the identity provider uses to sign and validate tokens.
    /// It must be identical to the URL used to retrieve the discovery document,
    /// and it must match the `iss` claim inside any ID token or access token.
    pub issuer: Url,
    /// The authorization endpoint.
    pub authorization_endpoint: Url,
    /// The token endpoint for requesting tokens.
    pub token_endpoint: Url,
    /// The URI for the JSON Web Key Set (JWKS).
    ///
    /// This URL should have the signing keys the Relying Party (RP) uses to validate signatures.
    pub jwks_uri: Url,
    /// The list of supported scopes.
    #[serde(default = "discovery_default_scopes")]
    pub scopes_supported: Vec<String>,
    // Note: There are several other useful values the spec requires
    // or recommends, which we might consider adding in the future.
}

/// Per [Provider Metadata][https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata],
/// the `scopes_supported` value must include at least the `openid` scope.
pub(crate) const DISCOVERY_REQUIRED_SCOPE: &str = "openid";

fn discovery_default_scopes() -> Vec<String> {
    vec![DISCOVERY_REQUIRED_SCOPE.to_string()]
}

impl Discovery {
    #[cfg(test)]
    pub(crate) fn new_for_test(issuer: Url) -> Self {
        Self {
            authorization_endpoint: issuer.join("/v1/authorize").unwrap(),
            token_endpoint: issuer.join("/v1/token").unwrap(),
            jwks_uri: issuer.join("/.well-known/jwks.json").unwrap(),
            scopes_supported: discovery_default_scopes(),
            issuer,
        }
    }
}

/// Fetch an OIDC discovery document from the given issuer URL.
///
/// This follows the OpenID Connect Discovery 1.0 specification,
/// [Section 4: "Obtaining OpenID Provider Configuration Information"][openid-discovery-section-4].
///
/// # Errors
///
/// This returns a [`DiscoveryError`] if attempting to fetch and deserialize the document fails,
/// of if the discovery document is invalid.
///
/// If the `issuer` parameter does not match the issuer URL in the discovery document,
/// this returns [`DiscoveryError::IssuerMismatch`].
/// OpenID Connect requires this to [prevent impersonation attacks][openid-impersonation].
/// The Relying Party must likewise check that the `iss` claim in ID tokens matches the issuer URL.
///
/// [openid-impersonation]: https://openid.net/specs/openid-connect-discovery-1_0.html#impersonation
/// [openid-discovery-section-4]: https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderConfig
pub(crate) async fn fetch_discovery(
    http: &reqwest::Client,
    issuer: impl AsRef<str>,
) -> Result<Discovery> {
    let issuer: Url = issuer.as_ref().parse()?;

    fetch_discovery_impl(
        http,
        issuer,
        #[cfg(not(any(test, feature = "_insecure-issuer-validation")))]
        ValidationStrategy::strict(),
        #[cfg(any(test, feature = "_insecure-issuer-validation"))]
        ValidationStrategy::insecure(),
    )
    .await
}

async fn fetch_discovery_impl(
    http: &reqwest::Client,
    issuer: Url,
    validator: impl ValidateIssuer,
) -> Result<Discovery> {
    validator.validate_issuer(&issuer)?;

    // `Url::join` strips the final path segment,
    // e.g. `/a/b` joined with `c/d` -> `/a/c/d`
    //
    // If that's something other than a slash, we'd lose it;
    // to prevent that, we manually extend the path.
    let discovery_url = {
        let mut url = issuer.clone();
        match url.path_segments_mut() {
            Ok(mut segments) => {
                segments.extend(&[".well-known", "openid-configuration"]);
            }
            Err(()) => {
                return Err(DiscoveryError::InvalidIssuer {
                    issuer: issuer.to_string(),
                    reason: "the issuer URL is not a valid URL".to_string(),
                });
            }
        }
        url
    };

    #[cfg(feature = "tracing")]
    tracing::info!(
        discovery_url = discovery_url.as_str(),
        "Fetching OIDC discovery document."
    );

    let discovery: Discovery = http
        .get(discovery_url.clone())
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    validator.validate_issuer(&discovery.issuer)?;

    if issuer != discovery.issuer {
        return Err(DiscoveryError::IssuerMismatch {
            document: discovery.issuer.into(),
            query: issuer.to_string(),
        });
    }

    if discovery
        .scopes_supported
        .iter()
        .all(|scope| scope != DISCOVERY_REQUIRED_SCOPE)
    {
        return Err(DiscoveryError::InvalidScopes(discovery.scopes_supported));
    }

    Ok(discovery)
}

/// A trait for validation during the OIDC discovery process.
///
/// This is used primarily to relax the restrictions during testing,
/// but in general, production use should follow the OpenID Connect Discovery 1.0 specification.
trait ValidateIssuer {
    fn validate_issuer(&self, issuer: &Url) -> Result<()>;
}

/// Validation strategy for an issuer URL.
#[derive(Debug, Clone, Copy)]
#[expect(clippy::struct_excessive_bools)]
struct ValidationStrategy {
    scheme: bool,
    host: bool,
    query: bool,
    fragment: bool,
}

impl Default for ValidationStrategy {
    fn default() -> Self {
        Self::strict()
    }
}

impl ValidationStrategy {
    /// Validate an issuer according to the OpenID Connect Discovery 1.0 specification,
    /// [Section 2: "OpenID Provider Issuer Discovery"][openid-discovery-section-2].
    ///
    /// Specifically, the following requirements are checked:
    ///
    /// - The scheme must be https.
    /// - There must be a host component.
    /// - No query nor fragments are allowed.
    ///
    /// [openid-discovery-section-2]: https://openid.net/specs/openid-connect-discovery-1_0.html#IssuerDiscovery
    const fn strict() -> Self {
        Self {
            scheme: true,
            host: true,
            query: true,
            fragment: true,
        }
    }

    /// Validate an issuer as [`ValidationStrategy::strict`], but skips the `scheme` check.
    #[cfg(any(test, feature = "_insecure-issuer-validation"))]
    const fn insecure() -> Self {
        Self {
            scheme: false,
            host: true,
            query: true,
            fragment: true,
        }
    }
}

impl ValidateIssuer for ValidationStrategy {
    /// Validate an issuer based on the [`ValidationStrategy`].
    ///
    /// # Errors
    ///
    /// This returns a [`DiscoveryError::InvalidIssuer`] if the issuer URL is invalid.
    fn validate_issuer(&self, issuer: &Url) -> Result<()> {
        if self.scheme && issuer.scheme() != "https" {
            return Err(issuer_err(issuer, "the issuer scheme is not https"));
        }

        if self.host && issuer.host_str().is_none_or(str::is_empty) {
            return Err(issuer_err(issuer, "the issuer has no host"));
        }

        if self.query && issuer.query().is_some() {
            return Err(issuer_err(issuer, "the issuer contains a query"));
        }

        if self.fragment && issuer.fragment().is_some() {
            return Err(issuer_err(issuer, "the issuer contains a fragment"));
        }

        Ok(())
    }
}

/// Helper function to generate an [`DiscoveryError::InvalidIssuer`] error from a URL.
fn issuer_err(issuer: &Url, reason: &str) -> DiscoveryError {
    DiscoveryError::InvalidIssuer {
        issuer: issuer.as_str().to_string(),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use httpmock::prelude::*;
    use rstest::rstest;

    fn http_client() -> reqwest::Client {
        reqwest::Client::builder().build().unwrap()
    }

    #[tokio::test]
    #[rstest]
    async fn test_discovery(#[values("", "/")] issuer_suffix: &str) {
        let mock_server = MockServer::start_async().await;

        let issuer = Url::parse(&mock_server.base_url()).unwrap();
        let expected = Discovery::new_for_test(issuer);

        let oidc_mock = mock_server
            .mock_async(|when, then| {
                when.method(GET).path("/.well-known/openid-configuration");
                then.status(200).json_body_obj(&expected);
            })
            .await;

        let actual = fetch_discovery_impl(
            &http_client(),
            mock_server.url(issuer_suffix).parse().unwrap(),
            ValidationStrategy::insecure(),
        )
        .await
        .expect("should fetch discovery document");

        assert_eq!(actual.token_endpoint, expected.token_endpoint);

        oidc_mock.assert_async().await;
    }

    #[rstest]
    fn test_validation_strategy_invalid(
        #[values(
            "http://example.com",           // not https
            "https://example.com?foo=bar",  // has query
            "https://example.com#foo=bar",  // has fragment
        )]
        issuer: &str,
    ) {
        let strategy = ValidationStrategy::strict();
        let issuer = Url::parse(issuer).unwrap();
        assert!(
            strategy.validate_issuer(&issuer).is_err(),
            "issuer: {issuer:?}"
        );
    }

    #[test]
    fn test_validation_strategy_invalid_host() {
        // An empty host isn't valid with https,
        // so normally this would be caught by the URL parser;
        // nevertheless, if some day our URL parser stops checking it for us,
        // we want to know an empty host still results in an error.
        let strategy = ValidationStrategy {
            scheme: false,
            ..ValidationStrategy::strict()
        };
        let issuer = Url::parse("foo:/./path").unwrap();
        assert!(
            strategy.validate_issuer(&issuer).is_err(),
            "issuer: {issuer:?}"
        );
    }

    #[rstest]
    fn test_validation_strategy_valid(
        #[values(
            "https://example.com",
            "https://example.com:80",
            "https://example.com/some/path",
            "https://example.com/some/path/with/slash/"
        )]
        issuer: &str,
    ) {
        let strategy = ValidationStrategy::strict();
        let issuer = Url::parse(issuer).unwrap();
        assert!(strategy.validate_issuer(&issuer).is_ok());
    }
}
