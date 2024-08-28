use qcs_api_client_common::configuration::ClientConfiguration;

#[tokio::test]
async fn test_token_refresh() {
    let configuration = ClientConfiguration::load_default()
        .expect("Should be able to load valid QCS configuration.");

    let fresh_tokens = configuration
        .refresh()
        .await
        .expect("Should be able to refresh token.");

    let access_token = configuration
        .get_bearer_access_token()
        .await
        .expect("Should be able to fetch recently refreshed token.");

    assert_eq!(fresh_tokens.access_token().unwrap(), access_token, "Testing that a newly refreshed token is not refreshed when fetching the token immediately after, implying that JWT validation is working as expected.");
}
