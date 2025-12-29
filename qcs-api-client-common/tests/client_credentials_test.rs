//! Client-credentials (i.e. service authorization) test

use qcs_api_client_common::configuration::{settings::AuthServer, tokens::ClientCredentials};

const VAR_NAME_ISSUER_URL: &str = "ISSUER_URL";
const VAR_NAME_CLIENT_ID: &str = "ISSUER_CLIENT_ID";
const VAR_NAME_CLIENT_SECRET: &str = "ISSUER_CLIENT_SECRET";

fn require_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("must set {key}"))
}

#[tokio::test]
async fn test_client_credentials() {
    let credentials = ClientCredentials::new(
        require_var(VAR_NAME_CLIENT_ID),
        require_var(VAR_NAME_CLIENT_SECRET),
    );
    let issuer = AuthServer::new(
        credentials.client_id.clone(),
        require_var(VAR_NAME_ISSUER_URL),
        None,
    );

    credentials
        .request_access_token(&issuer)
        .await
        .expect("could not get access token");
}
