//! Shared plumbing for the interactive `OAuth2` login flows.

use std::collections::BTreeSet;

use crate::configuration::{oidc::DISCOVERY_REQUIRED_SCOPE, settings::DEFAULT_LOGIN_SCOPES};

/// Resolve the set of scopes to request during an interactive login.
///
/// Uses the explicitly configured scopes if there are any, and [`DEFAULT_LOGIN_SCOPES`] otherwise.
/// [`DISCOVERY_REQUIRED_SCOPE`] is always included, since it is required to be supported and the
/// client relies on it.
pub(crate) fn resolve_scopes(configured_scopes: Option<Vec<String>>) -> BTreeSet<String> {
    let mut scopes = configured_scopes
        .filter(|scopes| !scopes.is_empty())
        .unwrap_or_else(|| DEFAULT_LOGIN_SCOPES.map(String::from).to_vec())
        .into_iter()
        .collect::<BTreeSet<_>>();
    scopes.insert(DISCOVERY_REQUIRED_SCOPE.to_string());
    scopes
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Stringified version of [`DEFAULT_LOGIN_SCOPES`]
    pub(crate) fn default_scope_string() -> String {
        resolve_scopes(None)
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Configured scopes are used verbatim, with [`DISCOVERY_REQUIRED_SCOPE`] added if missing.
    #[test]
    fn test_configured_scopes_are_used_verbatim() {
        let configured = vec!["custom:read".to_string(), "custom:write".to_string()];
        let scopes = resolve_scopes(Some(configured));

        assert_eq!(
            scopes.into_iter().collect::<Vec<_>>(),
            ["custom:read", "custom:write", DISCOVERY_REQUIRED_SCOPE]
        );
    }

    /// An explicitly empty scope list is treated as "unconfigured" rather than as "request only
    /// the one required scope".
    #[test]
    fn test_empty_configured_scopes_resolve_to_defaults() {
        assert_eq!(resolve_scopes(Some(Vec::new())), resolve_scopes(None));
    }
}
