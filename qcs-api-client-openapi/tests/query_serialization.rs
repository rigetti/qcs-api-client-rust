//! Verify how typed query parameters are serialized onto the wire.

use qcs_api_client_openapi::{
    apis::{configuration::Configuration, reservations_api},
    common::ClientConfiguration,
    common::configuration::{
        settings::AuthServer,
        tokens::{ExternallyManaged, OAuthSession},
    },
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Serve a single request on a local port, returning the base URL and a handle yielding the
/// request target (e.g. `/v1/path?query=value`).
async fn serve_once() -> (String, tokio::task::JoinHandle<String>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 8192];
        let n = socket.read(&mut buf).await.unwrap();
        let request = String::from_utf8_lossy(&buf[..n]).into_owned();
        let _ = socket
            .write_all(b"HTTP/1.1 500 x\r\ncontent-length: 0\r\nconnection: close\r\n\r\n")
            .await;
        request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap()
            .to_owned()
    });
    (base, handle)
}

/// A `DateTime` query parameter must be sent as RFC 3339 (`T` separator, no spaces), which is
/// what the API requires. `DateTime`'s `Display` impl is not RFC 3339.
#[tokio::test]
async fn date_time_query_param_is_rfc3339() {
    let (base, handle) = serve_once().await;
    let configuration = Configuration::with_qcs_config(
        ClientConfiguration::builder()
            .api_url(base)
            .oauth_session(Some(OAuthSession::from_externally_managed(
                ExternallyManaged::from_sync(|_| Ok("fake-token".to_owned())),
                AuthServer::default(),
                None,
            )))
            .build()
            .unwrap(),
    );
    let start = chrono::DateTime::parse_from_rfc3339("2026-10-22T23:25:36.460339+00:00").unwrap();

    let _ = reservations_api::find_available_reservations(
        &configuration,
        "Ankaa-3",
        start,
        "1h",
        None,
        None,
    )
    .await;

    let target = handle.await.unwrap();
    let (_, query) = target.split_once('?').unwrap();
    let value = query
        .split('&')
        .find_map(|pair| pair.strip_prefix("startTimeFrom="))
        .unwrap()
        .replace("%3A", ":")
        .replace("%2B", "+");
    let parsed = chrono::DateTime::parse_from_rfc3339(&value)
        .unwrap_or_else(|err| panic!("startTimeFrom {value:?} is not RFC 3339: {err}"));
    assert_eq!(parsed, start);
}
