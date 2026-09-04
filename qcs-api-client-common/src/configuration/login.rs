//! Shared plumbing for the interactive `OAuth2` login flows.

use std::collections::BTreeSet;

use crate::configuration::{oidc::DISCOVERY_REQUIRED_SCOPE, settings::PREFERRED_LOGIN_SCOPES};

/// Resolve the set of scopes to request during an interactive login.
///
/// Always includes [`DISCOVERY_REQUIRED_SCOPE`], mandated by the OIDC spec.
///
/// - If `configured_scopes` is provided, that's explicit user configuration; respect it.
/// - If `advertised_scopes` is provided, narrow to those also in [`PREFERRED_LOGIN_SCOPES`];
///   there may be many scopes but we only need some basic ones. Note that Cognito does not
///   advertise `offline_access` even though it supports refresh tokens, so we don't ask for it
///   even though we prefer it (and get the refresh token anyway!).
/// - If neither is provided, only request the minimum necessary [`DISCOVERY_REQUIRED_SCOPE`].
pub fn resolve_scopes(
    configured_scopes: Option<BTreeSet<String>>,
    advertised_scopes: Option<BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut scopes = configured_scopes
        .or_else(|| {
            advertised_scopes.map(|advertised_scopes| {
                advertised_scopes
                    .into_iter()
                    .filter(|scope| PREFERRED_LOGIN_SCOPES.contains(&scope.as_str()))
                    .collect()
            })
        })
        .unwrap_or_default();

    scopes.insert(DISCOVERY_REQUIRED_SCOPE.to_string());
    scopes
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    macro_rules! set {
        ($($v:expr),* $(,)?) => {
            [$($v,)*]
                .iter()
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>()
        };
    }

    /// Configured scopes are used verbatim, with [`DISCOVERY_REQUIRED_SCOPE`] added if missing.
    #[test]
    fn test_configured_scopes_are_used_verbatim() {
        let configured = set!["custom:read", "custom:write"];
        let expected = set!["custom:read", "custom:write", DISCOVERY_REQUIRED_SCOPE];
        let actual = resolve_scopes(Some(configured), None);

        assert_eq!(expected, actual);
    }

    /// A provider that says nothing about its scopes gets the minimum.
    #[test]
    fn test_unadvertised_scopes_fall_back_to_all_defaults() {
        let actual = resolve_scopes(None, None);
        let expected = set![DISCOVERY_REQUIRED_SCOPE];

        assert_eq!(expected, actual);
    }

    /// The Cognito case: a provider that supports refresh tokens but does not advertise (or
    /// accept) `offline_access` should not be asked for it, and extra scopes should be ignored.
    #[test]
    fn test_defaults_are_narrowed_to_advertised_scopes() {
        let cognito_scopes = set![
            DISCOVERY_REQUIRED_SCOPE,
            "email",
            "profile",
            "something_else"
        ];

        let expected = set![DISCOVERY_REQUIRED_SCOPE, "email", "profile"];
        let actual = resolve_scopes(None, Some(cognito_scopes.clone()));

        assert_eq!(expected, actual);
    }

    /// Configured scopes are a deliberate choice, so the advertised set does not narrow them.
    #[test]
    fn test_configured_scopes_are_not_narrowed() {
        let configured = set!["offline_access", "something_else"];
        let advertised = set!["email"];
        let expected = set![DISCOVERY_REQUIRED_SCOPE, "offline_access", "something_else"];
        let actual = resolve_scopes(Some(configured), Some(advertised));

        assert_eq!(expected, actual);
    }
}
