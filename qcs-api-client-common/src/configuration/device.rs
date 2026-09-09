//! The [Device Authorization Grant][rfc8628] (also called the "device code flow").
//!
//! Unlike the [PKCE flow][super::pkce], this does not need a local redirect listener, so:
//! - Does not require the `OAuth2` client to allowlist a `http://127.0.0.1:{port}` redirect URI.
//! - Is usable in environments where a browser cannot redirect back to this process, such as SSH
//!   sessions or cloud-hosted environments.
//!
//! However, it is not as widely supported as PKCE. Some identity providers (e.g. Cognito) do not
//! provide it without significant manual work. So this method is generally preferred to PKCE
//! but is not always available where PKCE is expected to be.
//!
//! [rfc8628]: https://datatracker.ietf.org/doc/html/rfc8628

use std::collections::BTreeSet;

use oauth2::{
    ClientId, DeviceAuthorizationUrl, DeviceCodeErrorResponse, DeviceCodeErrorResponseType,
    HttpClientError, RequestTokenError, Scope, StandardDeviceAuthorizationResponse,
    StandardErrorResponse, TokenUrl,
    basic::{BasicClient, BasicErrorResponseType},
};
use tokio_util::sync::CancellationToken;
use url::Url;

use crate::configuration::login::{LoginResponse, oauth_http_client, resolve_scopes};

/// Errors that can occur while trying to perform a device authorization login.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DeviceLoginError {
    #[error(transparent)]
    ReqwestClient(#[from] oauth2::reqwest::Error),
    #[error("Failed to request a device authorization code: {0}")]
    DeviceAuthorization(
        #[from]
        RequestTokenError<
            HttpClientError<oauth2::reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    /// Polling the token endpoint uses the device-specific error response, so that
    /// `authorization_pending` and `slow_down` are distinguishable from a real failure.
    #[error("Failed to exchange the device code for an access token: {0}")]
    RequestToken(
        #[from] RequestTokenError<HttpClientError<oauth2::reqwest::Error>, DeviceCodeErrorResponse>,
    ),
    #[error("The device authorization login was cancelled")]
    Cancelled,
    #[error(
        "The OAuth issuer does not advertise support for the device authorization grant. It must \
         publish a `device_authorization_endpoint` in its discovery document."
    )]
    NotSupported,
}

impl DeviceLoginError {
    /// Whether a PKCE login is still worth attempting after this error.
    ///
    /// A user who cancelled or refused the request meant it, so
    /// [`super::tokens::LoginFlowPreference::Auto`] should not send them to a browser instead.
    pub(crate) fn allows_pkce_fallback(&self) -> bool {
        match self {
            Self::Cancelled => false,
            Self::RequestToken(RequestTokenError::ServerResponse(response)) => {
                !matches!(response.error(), DeviceCodeErrorResponseType::AccessDenied)
            }
            _ => true,
        }
    }
}

/// How a device authorization login tells the user where to go and what code to enter.
pub(crate) enum DevicePrompt {
    /// Print the verification URI and user code, and try to open a browser at the URI.
    User,
    /// Report the user code back to the test rather than printing it.
    #[cfg(test)]
    Test(tokio::sync::mpsc::UnboundedSender<String>),
}

impl DevicePrompt {
    /// Show the user the verification URI and the code to enter there.
    async fn show(self, details: &StandardDeviceAuthorizationResponse) {
        match self {
            Self::User => {
                let verification_uri = details.verification_uri();

                println!(
                    "Login to QCS by going to {verification_uri} and entering the code: {user_code}",
                    user_code = details.user_code().secret(),
                );

                // A `verification_uri_complete` embeds the code, so a browser that can reach it
                // skips the code entry entirely.
                let browser_uri = details
                    .verification_uri_complete()
                    .map_or_else(|| verification_uri.to_string(), |uri| uri.secret().clone());

                // Opening a browser is a convenience, not a requirement. Some environments
                // may support opening a webpage but not redirecting back to localhost.
                _ = tokio::task::spawn_blocking(move || webbrowser::open(&browser_uri)).await;
            }
            #[cfg(test)]
            Self::Test(prompted_tx) => {
                _ = prompted_tx.send(details.user_code().secret().clone());
            }
        }
    }
}

/// The request parameters for a device authorization login.
pub(crate) struct DeviceLoginRequest {
    /// The oauth2 client ID to use for the login.
    pub(crate) client_id: String,
    /// The endpoint to exchange the device code for tokens at.
    pub(crate) token_endpoint: Url,
    /// The device authorization endpoint to request a device code from.
    pub(crate) device_authorization_endpoint: Url,
    /// The scopes configured for the auth server, if any.
    pub(crate) scopes: Option<BTreeSet<String>>,
    /// The scopes the issuer's discovery document advertises, if any.
    ///
    /// See [`resolve_scopes`] for how this narrows the request when `scopes` is [`None`].
    pub(crate) advertised_scopes: Option<BTreeSet<String>>,
    /// How to tell the user where to go and what code to enter.
    pub(crate) prompt: DevicePrompt,
}

/// Launch a device authorization login, requiring the user to enter a code at a verification URI.
pub(crate) async fn device_login(
    cancel_token: CancellationToken,
    request: DeviceLoginRequest,
) -> Result<LoginResponse, DeviceLoginError> {
    let DeviceLoginRequest {
        client_id,
        token_endpoint,
        device_authorization_endpoint,
        scopes,
        advertised_scopes,
        prompt,
    } = request;

    let client = BasicClient::new(ClientId::new(client_id))
        .set_token_uri(TokenUrl::from_url(token_endpoint))
        .set_device_authorization_url(DeviceAuthorizationUrl::from_url(
            device_authorization_endpoint,
        ));

    let scopes = resolve_scopes(scopes, advertised_scopes);

    let http_client = oauth_http_client()?;

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .add_scopes(scopes.into_iter().map(Scope::new))
        .request_async(&http_client)
        .await?;

    prompt.show(&details).await;

    // `request_async` is the polling loop, not a single request: it keeps hitting the token
    // endpoint until the user approves or the device code expires, honoring the response
    // params `interval + slow_down + expires_in`. The crate docs only imply this, via
    // `DeviceAccessTokenRequest::set_max_backoff_interval`.
    cancel_token
        .run_until_cancelled(client.exchange_device_access_token(&details).request_async(
            &http_client,
            tokio::time::sleep,
            None,
        ))
        .await
        .ok_or(DeviceLoginError::Cancelled)?
        .map_err(DeviceLoginError::RequestToken)
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use oauth2::TokenResponse;
    use oauth2_test_server::{Client as TestClient, IssuerConfig, OAuthTestServer};
    use rstest::rstest;
    use serde_json::json;
    use tokio_util::sync::CancellationToken;

    use crate::configuration::{
        login::tests::default_scope_string, oidc::DEVICE_CODE_GRANT_TYPE,
        secrets::SecretAccessToken, tokens::insecure_validate_token_exp,
    };

    use super::*;

    /// The path the canned device authorization response is served from.
    const DEVICE_AUTHORIZE_PATH: &str = "/v1/device/authorize";

    /// The lifetime advertised for the mocked device code, in seconds.
    ///
    /// The polling loop gives up once this elapses, and it is the only thing bounding that loop.
    /// Keeping it short means a test that never gets its approval fails with the underlying OAuth
    /// error in a few seconds, instead of polling until the harness kills it.
    const DEVICE_CODE_EXPIRES_IN_SECS: u64 = 15;

    /// Start an OAuth test server with a client that is allowed to use the device grant.
    async fn start_device_server() -> (OAuthTestServer, TestClient) {
        let server = OAuthTestServer::start_with_config(IssuerConfig::default()).await;
        let client = server
            .register_client(json!({
                "scope": default_scope_string(),
                "grant_types": [DEVICE_CODE_GRANT_TYPE],
                "client_name": "device-flow-test",
            }))
            .await;
        (server, client)
    }

    /// Create a device code on the OAuth test server, returning `(device_code, user_code)`.
    ///
    /// The test has to do this itself: the test server offers no way to enumerate or auto-approve
    /// device codes, and the client under test never reveals the device code it is polling with.
    async fn create_device_code(server: &OAuthTestServer, client_id: &str) -> (String, String) {
        let response = server
            .http
            .post(format!("{}/device/code", server.issuer()))
            .form(&[("client_id", client_id), ("scope", &default_scope_string())])
            .send()
            .await
            .expect("device code request should succeed");

        let body: serde_json::Value = response
            .json()
            .await
            .expect("device code response should be JSON");

        let field = |name: &str| {
            body.get(name)
                .and_then(serde_json::Value::as_str)
                .unwrap_or_else(|| panic!("device code response should have a `{name}`: {body}"))
                .to_string()
        };

        (field("device_code"), field("user_code"))
    }

    /// A device authorization the user approves should yield usable tokens.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_device_login() {
        let (oauth_server, client) = start_device_server().await;
        let (device_code, user_code) = create_device_code(&oauth_server, &client.client_id).await;

        // The test server does not publish a `device_authorization_endpoint`, and a test cannot
        // learn the device code a client requested, so the canned response hands the client the
        // code created above. Its polling then runs against a real device authorization.
        let mock_server = MockServer::start_async().await;
        let device_authorize_mock = mock_server
            .mock_async(|when, then| {
                when.method(POST)
                    .path(DEVICE_AUTHORIZE_PATH)
                    .form_urlencoded_tuple("client_id", &client.client_id)
                    .form_urlencoded_tuple("scope", default_scope_string());
                then.status(200).json_body(json!({
                    "device_code": device_code,
                    "user_code": user_code,
                    "verification_uri": format!("{}/device", oauth_server.issuer()),
                    "expires_in": DEVICE_CODE_EXPIRES_IN_SECS,
                    // Poll fast, so waiting for approval doesn't make the test slow.
                    "interval": 1,
                }));
            })
            .await;

        let (prompted_tx, mut prompted_rx) = tokio::sync::mpsc::unbounded_channel();
        let request = DeviceLoginRequest {
            client_id: client.client_id.clone(),
            token_endpoint: format!("{}/device/token", oauth_server.issuer())
                .parse()
                .unwrap(),
            device_authorization_endpoint: mock_server.url(DEVICE_AUTHORIZE_PATH).parse().unwrap(),
            scopes: None,
            advertised_scopes: None,
            prompt: DevicePrompt::Test(prompted_tx),
        };

        // Approve only once the user has been prompted, so that the `authorization_pending`
        // responses the token endpoint returns until then are handled rather than treated as
        // failures.
        let (response, prompted_user_code) =
            tokio::join!(device_login(CancellationToken::new(), request), async {
                let prompted = prompted_rx
                    .recv()
                    .await
                    .expect("the login should prompt the user");
                oauth_server
                    .approve_device_code(&device_code, "device-test-user")
                    .await;
                prompted
            });

        let response = response.expect("device authorization login should succeed");

        device_authorize_mock.assert_async().await;
        assert_eq!(prompted_user_code, user_code);

        let access_token = SecretAccessToken::from(response.access_token().secret().clone());
        insecure_validate_token_exp(&access_token).expect("access token should be valid");
        assert!(
            response.refresh_token().is_some(),
            "the device token endpoint should have returned a refresh token"
        );
    }

    /// A token endpoint error response carrying the given error code.
    fn token_error(error: DeviceCodeErrorResponseType) -> DeviceLoginError {
        DeviceLoginError::RequestToken(RequestTokenError::ServerResponse(
            DeviceCodeErrorResponse::new(error, None, None),
        ))
    }

    /// A denial or a cancellation is the user's final answer; any other failure is worth a retry
    /// over PKCE, including a client that isn't allowed the device grant.
    #[rstest]
    #[case(DeviceLoginError::Cancelled, false)]
    #[case(token_error(DeviceCodeErrorResponseType::AccessDenied), false)]
    #[case(token_error(DeviceCodeErrorResponseType::ExpiredToken), true)]
    #[case(
        token_error(DeviceCodeErrorResponseType::Basic(
            BasicErrorResponseType::UnauthorizedClient
        )),
        true
    )]
    fn test_allows_pkce_fallback(#[case] error: DeviceLoginError, #[case] expected: bool) {
        assert_eq!(error.allows_pkce_fallback(), expected, "error: {error}");
    }
}
