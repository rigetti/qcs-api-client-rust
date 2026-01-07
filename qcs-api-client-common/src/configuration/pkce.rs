use std::{collections::HashSet, convert::Infallible};

use http::{Response, StatusCode};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields, HttpClientError,
    PkceCodeChallenge, RedirectUrl, RequestTokenError, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenUrl,
};

use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use url::form_urlencoded;

use crate::configuration::oidc::{Discovery, DISCOVERY_REQUIRED_SCOPE};

/// The scheme for the redirect URL.
const PKCE_REDIRECT_URL_SCHEME: &str = "http";

/// The origin for the redirect server hosted locally.
///
/// IMPORTANT: The oauth2 client must allow sign-in redirects to `{PKCE_REDIRECT_URL_SCHEME}://{PKCE_REDIRECT_URL_ORIGIN}:{redirect_port}`,
/// where the `redirect_port` is the port specified in the [`PkceLoginRequest`].
const PKCE_REDIRECT_URL_ORIGIN: &str = "127.0.0.1";

/// The default port for the redirect server hosted locally.
const PKCE_REDIRECT_URL_DEFAULT_PORT: u16 = 8484;

fn format_redirect_url(port: u16) -> RedirectUrl {
    RedirectUrl::from_url(
        format!("{PKCE_REDIRECT_URL_SCHEME}://{PKCE_REDIRECT_URL_ORIGIN}:{port}")
            .parse()
            .expect("well-formed URL should parse"),
    )
}

/// Errors that can occur while trying to perform a PKCE login.
#[derive(Debug, thiserror::Error)]
pub enum PkceLoginError {
    #[error(transparent)]
    RedirectListenerSpawnError(#[from] RedirectListenerSpawnError),
    #[error(transparent)]
    RedirectListenerError(#[from] RedirectListenerError),
    #[error(transparent)]
    ReqwestClient(#[from] reqwest::Error),
    #[error("Error joining redirect listener task: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("The redirect response's verifier state doesn't match the expected values")]
    CodeChallengeMismatch,
    #[error("Failed to exchange authorization code for token: {0}")]
    RequestToken(
        #[from]
        RequestTokenError<
            HttpClientError<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
}

/// The response returned by a Authorization Code Exchange following a successful PKCE login.
pub(crate) type PkceLoginResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

/// The request parameters for a PKCE login.
pub(crate) struct PkceLoginRequest {
    /// The oauth2 client ID to use for the PKCE login.
    pub(crate) client_id: String,
    /// The port to use for the redirect server.
    /// If `None`, the default port [`PKCE_REDIRECT_URL_DEFAULT_PORT`] will be used.
    ///
    /// IMPORTANT: The oauth2 client must allow sign-in redirects to `http://{PKCE_REDIRECT_ORIGIN}:{redirect_port}`.
    pub(crate) redirect_port: Option<u16>,
    /// The discovery document to use for the PKCE login.
    pub(crate) discovery: Discovery,
    /// The scopes to request in the token authorization to request.
    /// If `None`, all scopes from [`Discovery::scopes_supported`] will be requested.
    pub(crate) scopes: Option<Vec<String>>,
}

/// Launch a PKCE login, requiring the user to authenticate via browser.
pub(crate) async fn pkce_login(
    cancel_token: CancellationToken,
    request: PkceLoginRequest,
) -> Result<PkceLoginResponse, PkceLoginError> {
    let redirect_port = request
        .redirect_port
        .unwrap_or(PKCE_REDIRECT_URL_DEFAULT_PORT);

    let RedirectListener {
        redirect_url,
        join_handle,
    } = RedirectListener::spawn(cancel_token, redirect_port).await?;

    let client = BasicClient::new(ClientId::new(request.client_id))
        .set_auth_uri(AuthUrl::from_url(request.discovery.authorization_endpoint))
        .set_token_uri(TokenUrl::from_url(request.discovery.token_endpoint))
        .set_redirect_uri(redirect_url);

    let scopes = {
        let mut unique_scopes = request
            .scopes
            .unwrap_or(request.discovery.scopes_supported)
            .into_iter()
            .collect::<HashSet<_>>();
        unique_scopes.insert(DISCOVERY_REQUIRED_SCOPE.to_string());
        unique_scopes
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .set_pkce_challenge(pkce_challenge)
        .add_scopes(scopes.into_iter().map(Scope::new))
        .url();

    if cfg!(test) {
        // Tests are headless, and should use an oauth2 request that does not require entering credentials.
        let client = reqwest::Client::new();
        println!("Requesting auth URL: {auth_url}");
        client.get(auth_url).send().await?.error_for_status()?;
    } else {
        println!("Login to QCS by going to: {auth_url}");

        // Attempt to open the URL in the default browser, or notify the user to do so manually.
        if let Err(error) = webbrowser::open(auth_url.as_ref()) {
            eprintln!(
                "Failed to open URL in the default browser, please open it manually: {error}",
            );
        }
    }

    let CodeStatePair { code, state } = join_handle.await??;

    if state.secret() != csrf_token.secret() {
        return Err(PkceLoginError::CodeChallengeMismatch);
    }

    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let token_result = client
        .exchange_code(code)
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    Ok(token_result)
}

/// The code and state parameters returned by the redirect server.
struct CodeStatePair {
    code: AuthorizationCode,
    state: CsrfToken,
}

impl CodeStatePair {
    /// Parses the code and state parameters from the query string of a URL,
    /// e.g. `code=...&state=...`.
    pub(crate) fn from_query(query: &str) -> Option<Self> {
        let mut code = None;
        let mut state = None;
        for (key, value) in form_urlencoded::parse(query.as_bytes()) {
            match key.as_ref() {
                "code" => code = Some(value),
                "state" => state = Some(value),
                _ => {}
            }
            if code.is_some() && state.is_some() {
                break;
            }
        }

        match (code, state) {
            (Some(code), Some(state)) => Some(Self {
                code: AuthorizationCode::new(code.to_string()),
                state: CsrfToken::new(state.to_string()),
            }),
            _ => None,
        }
    }
}

/// Errors that can occur while trying to spawn a [`RedirectListener`].
#[derive(Debug, thiserror::Error)]
#[error("Failed to spawn redirect listener: {0}")]
pub struct RedirectListenerSpawnError(#[from] std::io::Error);

/// Errors that can occur while handling a redirect request from the OAuth authorization server,
/// in the context of a [`RedirectListener`]'s background thread.
#[derive(Debug, thiserror::Error)]
pub enum RedirectListenerError {
    #[error("Failed to handle redirect request: {0}")]
    HandlerError(#[from] HandlerError),
    #[error("The PKCE redirect listener was cancelled")]
    Cancelled,
}

/// A local redirect server that attempts to listen for a single request, which should be the
/// code and state parameters returned by the OAuth authorization server.
struct RedirectListener {
    /// The OAuth authorization server will call this URL with the authorization
    /// code and verifier state when the user has authenticated.
    redirect_url: RedirectUrl,
    /// The background task that handles the redirect request, which can be joined/awaited to retrieve the code and state parameters once the redirect completes.
    join_handle: tokio::task::JoinHandle<Result<CodeStatePair, RedirectListenerError>>,
}

impl RedirectListener {
    /// Spawns a [`RedirectListener`], which listens for a single request from the OAuth authorization server
    /// on a background thread that can be joined to via [`RedirectListener::join_handle`].
    async fn spawn(
        cancel: CancellationToken,
        port: u16,
    ) -> Result<Self, RedirectListenerSpawnError> {
        let bind_addr = format!("127.0.0.1:{port}");
        let listener = TcpListener::bind(&bind_addr).await?;
        let bind_port = listener.local_addr()?.port();

        let redirect_url = format_redirect_url(bind_port);

        let join_handle = tokio::spawn(async move {
            cancel
                .run_until_cancelled_owned(handler(listener))
                .await
                .map_or(Err(RedirectListenerError::Cancelled), |result| {
                    result.map_err(RedirectListenerError::HandlerError)
                })
        });

        Ok(Self {
            redirect_url,
            join_handle,
        })
    }
}

/// Errors that can occur while handling a redirect request from the OAuth authorization server.
#[derive(Debug, thiserror::Error)]
pub enum HandlerError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error("Expected 'code' and 'state' query string parameters, but got: {0:?}")]
    ResponseCodeStatePair(Option<String>),
}

/// Handles a single request from a [`TcpListener`], expecting a response with code and state query string parameters.
async fn handler(listener: TcpListener) -> Result<CodeStatePair, HandlerError> {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let service = service_fn(move |req| {
        let tx = tx.clone();
        async move {
            let query = req.uri().query().map(str::to_string);

            let response = if let Some(pair) = query.as_deref().and_then(CodeStatePair::from_query)
            {
                if tx.send(pair).await.is_ok() {
                    build_response(
                        StatusCode::OK,
                        "Authorization complete. You may close this window.",
                    )
                } else {
                    const MESSAGE: &str =
                        "Authorization failed, the listener is done processing requests.";
                    eprintln!("{MESSAGE}");
                    build_response(StatusCode::BAD_REQUEST, MESSAGE)
                }
            } else {
                let error = HandlerError::ResponseCodeStatePair(query);
                build_response(StatusCode::BAD_REQUEST, error.to_string())
            };

            Ok::<_, Infallible>(response)
        }
    });

    // Serve the connection
    let conn = http1::Builder::new().serve_connection(io, service);

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(error) = conn.await {
            eprintln!("Error serving connection: {error}");
        }
    });

    // Wait for the result from the service.
    rx.recv()
        .await
        .ok_or(HandlerError::ResponseCodeStatePair(None))
}

/// Creates an HTTP response with a simple HTML page.
fn build_response(status: StatusCode, message: impl std::fmt::Display) -> Response<Full<Bytes>> {
    let reason = status.canonical_reason().unwrap_or_default();
    let style = "width: 100%; height: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center; font-family: sans-serif;";
    let body =
        format!("<html><body style=\"{style}\"><h1>{reason}</h1><p>{message}</p></body></html>");

    Response::builder()
        .status(status)
        .header("Content-Type", "text/html")
        .body(Full::new(Bytes::from(body)))
        .expect("should be well-formed response")
}

#[cfg(test)]
pub(in crate::configuration) mod tests {
    use oauth2::TokenResponse;
    use oauth2_test_server::{Client, IssuerConfig, OAuthTestServer};

    use crate::configuration::{
        oidc::{fetch_discovery, DISCOVERY_REQUIRED_SCOPE},
        secrets::SecretAccessToken,
        tokens::insecure_validate_token_exp,
    };

    use super::*;

    /// A test harness for the PKCE flow, containing the OAuth test server, client, and discovery document.
    ///
    /// IMPORTANT: for now, mark your tests with `#[serial_test::serial(oauth2_test_server)]` (and
    /// for `nextest`, use the `oauth2_test_server` test-group) to ensure that they run serially, as
    /// the OAuth test server does not (yet) support wildcarding the redirect URI, so we cannot yet
    /// allow the `pkce_login` listener to bind to any port.
    pub(in crate::configuration) struct PkceTestServerHarness {
        pub server: OAuthTestServer,
        pub client: Client,
        pub discovery: Discovery,
        pub redirect_port: u16,
    }

    impl PkceTestServerHarness {
        pub(in crate::configuration) async fn new() -> Self {
            let server = OAuthTestServer::start_with_config(IssuerConfig {
                scheme: PKCE_REDIRECT_URL_SCHEME.to_string(),
                host: PKCE_REDIRECT_URL_ORIGIN.to_string(),
                ..Default::default()
            })
            .await;

            let discovery = fetch_discovery(&reqwest::Client::new(), server.issuer())
                .await
                .unwrap();

            let redirect_url = format_redirect_url(PKCE_REDIRECT_URL_DEFAULT_PORT);
            let client = server.register_client(serde_json::json!({
                "scope": DISCOVERY_REQUIRED_SCOPE,
                "redirect_uris": [redirect_url],
                "client_name": "PkceTestServerHarness"
            }));

            Self {
                server,
                client,
                discovery,
                redirect_port: PKCE_REDIRECT_URL_DEFAULT_PORT,
            }
        }
    }

    #[tokio::test]
    #[serial_test::serial(oauth2_test_server)]
    async fn test_pkce_login() {
        let PkceTestServerHarness {
            server,
            client,
            discovery,
            redirect_port,
        } = PkceTestServerHarness::new().await;

        let request = PkceLoginRequest {
            client_id: client.client_id,
            redirect_port: Some(redirect_port),
            discovery,
            scopes: None,
        };

        let token_result = pkce_login(CancellationToken::new(), request)
            .await
            .expect("pkce_login should succeed");

        let access_token = SecretAccessToken::from(token_result.access_token().secret().clone());

        insecure_validate_token_exp(&access_token).expect("token should be valid");

        drop(server);
    }
}
