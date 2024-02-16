use eyre::Result;
use qcs_api_client_grpc::{
    channel::{get_channel, parse_uri, wrap_channel_with},
    client_configuration::ClientConfiguration,
    services::translation::{
        translate_quil_to_encrypted_controller_job_request::NumShots,
        translation_client::TranslationClient, TranslateQuilToEncryptedControllerJobRequest,
        TranslateQuilToEncryptedControllerJobResponse,
    },
};
use qcs_api_client_openapi::{
    apis::configuration::Configuration, apis::quantum_processors_api::list_quantum_processors,
    models::ListQuantumProcessorsResponse,
};
use serial_test::serial;

fn set_https_proxy() {
    std::env::set_var("HTTPS_PROXY", "socks5://127.0.0.1:49818");
}

/// Make a gRPC translation request to the live QCS API using proxy configuration.
///
/// Loads a default [`ClientConfiguration`] and thus relies on the presence of
/// QCS configuration files.
async fn request_translation() -> Result<TranslateQuilToEncryptedControllerJobResponse> {
    let config = ClientConfiguration::load_default().await?;

    let uri = parse_uri(config.grpc_api_url())?;
    let mut client = TranslationClient::new(wrap_channel_with(get_channel(uri)?, config));

    let request = TranslateQuilToEncryptedControllerJobRequest {
        num_shots: Some(NumShots::NumShotsValue(1)),
        quantum_processor_id: String::from("Aspen-M-3"),
        quil_program: String::from("DECLARE ro BIT"),
        options: None,
    };

    let response = client
        .translate_quil_to_encrypted_controller_job(request)
        .await?
        .into_inner();

    Ok(response)
}

/// Make an https request to list quantum processors.
async fn request_list_quantum_processors() -> Result<ListQuantumProcessorsResponse> {
    let config = Configuration::new().await?;

    let response = list_quantum_processors(&config, None, None).await?;

    Ok(response)
}

#[cfg(not(feature = "docker"))]
mod test_in_process {
    use std::sync::Arc;
    use std::time::Duration;

    use eyre::{eyre, Result};
    use socks5_server::{auth::NoAuth, Server};
    use tokio::net::TcpListener;

    use super::*;

    /// Contains a socks5 proxy server.
    struct Proxy(Server<()>);

    impl Proxy {
        /// Create a new socks server that binds to an OS-assigned port.
        async fn new() -> Result<Self> {
            let auth = Arc::new(NoAuth);
            let listener = TcpListener::bind("127.0.0.1:49818").await?;
            let server = Server::new(listener, auth);
            Ok(Self(server))
        }

        /// Returns `Ok(())` if a connection was successfully made to the service.
        /// Waits for 5 seconds maximum.
        async fn accept_one_connection(self) -> Result<()> {
            tokio::time::timeout(Duration::from_secs(5), async {
                match self.0.accept().await {
                    Ok((mut conn, _)) => {
                        let _ = conn.close().await;
                        Ok(())
                    }
                    Err(err) => Err(eyre!("Server could not accept connections: {}", err)),
                }
            })
            .await
            .map_err(|err| eyre!("{}", err.to_string()))?
        }
    }

    /// The proxy should be called if system proxy variables are set.
    #[tokio::test]
    #[serial]
    async fn test_openapi_proxy_connects() -> Result<()> {
        set_https_proxy();
        let proxy = Proxy::new().await?;
        let serve = tokio::spawn(proxy.accept_one_connection());

        // make request but do not expect it to fulfill, only that proxy receives it
        let _ = request_list_quantum_processors().await;

        // check that the proxy received a connection.
        serve.await??;

        Ok(())
    }

    /// The proxy should be called if system proxy variables are set.
    #[tokio::test]
    #[serial]
    async fn test_grpc_proxy_connects() -> Result<()> {
        set_https_proxy();
        let proxy = Proxy::new().await?;
        let serve = tokio::spawn(proxy.accept_one_connection());

        // make request but do not expect it to fulfill, only that proxy receives it
        let _ = request_translation().await;

        // check that the proxy received a connection.
        serve.await??;

        Ok(())
    }
}

#[cfg(feature = "docker")]
mod test_docker {
    use super::*;

    #[tokio::test]
    #[serial]
    async fn test_grpc_proxy() -> Result<()> {
        set_https_proxy();
        // the translation result comes back correctly
        let _ = request_translation()
            .await
            .expect_err("bad program should fail translation.");

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_openapi_proxy() -> Result<()> {
        set_https_proxy();
        // the translation result comes back correctly
        let _ = request_list_quantum_processors().await?;

        Ok(())
    }
}
