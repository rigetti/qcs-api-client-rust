//! End-to-end access token refresh test

use std::time::Duration;

use qcs_api_client_common::configuration::{
    secrets::{SecretAccessToken, SECRETS_READ_ONLY_VAR},
    ClientConfiguration, ConfigSource,
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use toml_edit::DocumentMut;

#[tokio::test]
async fn test_token_refresh() {
    let configuration = ClientConfiguration::load_default()
        .expect("Should be able to load valid QCS configuration.");

    let before_refresh = OffsetDateTime::now_utc();

    let fresh_tokens = configuration
        .refresh()
        .await
        .expect("Should be able to refresh token.");

    let access_token = configuration
        .get_bearer_access_token()
        .await
        .expect("Should be able to fetch recently refreshed token.");

    assert_eq!(fresh_tokens.access_token().unwrap(), &access_token, "Testing that a newly refreshed token is not refreshed when fetching the token immediately after, implying that JWT validation is working as expected.");

    tokio::time::sleep(Duration::from_secs_f64(2.0)).await;

    // If the configuration is associated with a profile in secrets.toml, the access_token should
    // be updated in the file.
    if let ConfigSource::File {
        settings_path: _,
        secrets_path,
    } = configuration.source()
    {
        if let Ok(ro_env) = std::env::var(SECRETS_READ_ONLY_VAR) {
            if matches!(ro_env.to_lowercase().as_str(), "true" | "yes" | "1") {
                // In this case, the file will *not* be updated.
                return;
            }
        }
        let toml = std::fs::read_to_string(secrets_path)
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();

        if let Some(token_payload) = toml
            .get("credentials")
            .and_then(|credentials| credentials.get(configuration.profile()))
            .and_then(|profile| profile.get("token_payload"))
        {
            assert_eq!(
                token_payload
                    .get("access_token")
                    .unwrap()
                    .as_str()
                    .map(str::to_string)
                    .map(SecretAccessToken::from),
                Some(access_token)
            );

            assert!(
                OffsetDateTime::parse(
                    token_payload.get("updated_at").unwrap().as_str().unwrap(),
                    &Rfc3339
                )
                .unwrap()
                    > before_refresh
            );
        }
    }
}
