use eyre::Result;
use qcs_api_client_grpc::{
    channel::{get_channel, parse_uri, wrap_channel_with},
    client_configuration::ClientConfiguration,
    services::translation::{
        translation_client::TranslationClient, GetQuantumProcessorQuilCalibrationProgramRequest,
    },
};
use qcs_api_client_openapi::{
    apis::configuration::Configuration, apis::quantum_processors_api::list_quantum_processors,
    models::ListQuantumProcessorsResponse,
};
use serial_test::serial;

/// Fetch the calibration program for the given QPU,
/// optionally wrapping the client with additional channel wrappers.
macro_rules! get_calibration_program_with_client_channels {
    ($qpu_id:expr $(, $channel_wrapper_fn:ident)*) => {async {
        let config = ClientConfiguration::load_default().await?;
        let uri = parse_uri(config.grpc_api_url())?;
        let channel = wrap_channel_with(get_channel(uri)?, config);

        $(let channel = $channel_wrapper_fn(channel);)*

        let mut client = TranslationClient::new(channel);

        let response = client
            .get_quantum_processor_quil_calibration_program(
                GetQuantumProcessorQuilCalibrationProgramRequest {
                    quantum_processor_id: String::from($qpu_id),
                },
            )
            .await?;

        Ok::<_, eyre::Report>(response)
    }};
}

/// Fetch the publicly available list of quantum processors.
async fn request_list_quantum_processors() -> Result<ListQuantumProcessorsResponse> {
    let config = Configuration::new().await?;
    let response = list_quantum_processors(&config, None, None).await?;
    Ok(response)
}

#[cfg(not(feature = "docker"))]
mod test_in_process {
    use super::*;

    use std::sync::Arc;
    use std::time::Duration;

    use eyre::{eyre, Result};
    use socks5_server::{auth::NoAuth, Server};
    use tokio::net::TcpListener;

    async fn with_proxy<F, O>(f: F) -> Result<()>
    where
        F: FnOnce() -> O,
        O: std::future::Future<Output = Result<()>>,
    {
        let auth = Arc::new(NoAuth);
        let listener = TcpListener::bind("127.0.0.1:1080").await?;

        let server = Server::new(listener, auth);
        let proxy_host = server.local_addr()?.to_string();

        // spawn a background task to accept a single connection
        let handle = tokio::spawn(tokio::time::timeout(Duration::from_secs(5), async move {
            match server.accept().await {
                Ok((mut conn, _)) => {
                    let _ = conn.close().await;
                    Ok(())
                }
                Err(err) => Err(eyre!("server could not accept connections: {}", err)),
            }
        }));

        let https_proxy = format!("socks5://{proxy_host}");
        std::env::set_var("HTTPS_PROXY", https_proxy);

        f().await?;

        handle.await??
    }

    /// The proxy should be called if system proxy variables are set.
    #[tokio::test]
    #[serial]
    async fn test_openapi_proxy_connects() -> Result<()> {
        with_proxy(|| async {
            // make request but don't care about the response,
            // only that the proxy receives the request.
            let _ = request_list_quantum_processors().await;

            Ok(())
        })
        .await
    }

    /// The proxy should be called if system proxy variables are set.
    #[tokio::test]
    #[serial]
    async fn test_grpc_proxy_connects() -> Result<()> {
        with_proxy(|| async {
            // make request but don't care about the response,
            // only that the proxy receives the request.
            let _ = get_calibration_program_with_client_channels!("Aspen-11").await;

            Ok(())
        })
        .await
    }
}

#[cfg(feature = "docker")]
mod test_in_docker {
    use qcs_api_client_grpc::channel::wrap_channel_with_grpc_web;

    use super::*;

    fn set_socks5_proxy() {
        let socks5_url =
            std::env::var("SOCKS5_URL").unwrap_or_else(|_| "socks5://localhost:1080".to_string());
        std::env::set_var("HTTP_PROXY", &socks5_url);
        std::env::set_var("HTTPS_PROXY", &socks5_url);
    }

    fn set_squid_proxy() {
        let squid_url =
            std::env::var("SQUID_URL").unwrap_or_else(|_| "http://localhost:3128".to_string());
        std::env::set_var("HTTP_PROXY", &squid_url);
        std::env::set_var("HTTPS_PROXY", &squid_url);
    }

    #[tokio::test]
    #[serial]
    async fn test_grpc_proxy_socks5() {
        set_socks5_proxy();
        match get_calibration_program_with_client_channels!("Aspen-11").await {
            Ok(_) => {}
            Err(err) if err.to_string().contains("no calibration found") => {
                // the only acceptable error, the request completed but
                // the env doesn't recognize the QPU.
            }
            Err(err) => panic!("{err}"),
        };
    }

    #[tokio::test]
    #[serial]
    async fn test_grpc_proxy_squid() {
        set_squid_proxy();
        match get_calibration_program_with_client_channels!("Aspen-11", wrap_channel_with_grpc_web)
            .await
        {
            Ok(_) => {}
            Err(err) if err.to_string().contains("no calibration found") => {
                // the only acceptable error, the request completed but
                // the env doesn't recognize the QPU.
            }
            Err(err) => panic!("{err}"),
        };
    }

    #[tokio::test]
    #[serial]
    async fn test_openapi_proxy_socks5() {
        set_socks5_proxy();
        let _ = request_list_quantum_processors()
            .await
            .expect("request should complete through proxy");
    }

    #[tokio::test]
    #[serial]
    async fn test_openapi_proxy_squid() {
        set_squid_proxy();
        let _ = request_list_quantum_processors()
            .await
            .expect("request should complete through proxy");
    }
}
