//! Helper macro and dedicated module for secret string types.
//! Avoid exporting this macro, since defining the secret here allows
//! us to make the inner value private and only expose the values via dedicated methods we want.
//! Define the secret strings here and re-export them in the appropriate modules.
#![allow(
    non_local_definitions,
    unreachable_pub,
    dead_code,
    reason = "necessary for pyo3::pymethods"
)]

use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

#[cfg(feature = "python")]
use crate::{impl_eq, impl_repr};

/// Builds a type that wraps [`Cow<'static, str>`] which helps prevent values
/// from being accidentally viewed in e.g. in debug or log output.
macro_rules! make_secret_string {
    (
       $(#[$attr:meta])*
       $name:ident
    ) => {
        $(#[$attr])*
        #[cfg_attr(feature = "python", ::pyo3::pyclass)]
        #[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
        #[serde(transparent)]
        pub struct $name(Cow<'static, str>);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                const NAME: &str = stringify!($name);
                let len = self.0.len();
                write!(f, "{NAME}(<REDACTED len: {len}>)")
            }
        }

        impl<T: Into<Cow<'static, str>>> From<T> for $name {
            fn from(value: T) -> Self {
                Self(value.into())
            }
        }

        impl $name {
            #[must_use]
            /// Check if the secret is an empty value
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[must_use]
            /// Get the inner secret contents, which removes the protection against accidentally exposing the value.
            pub fn secret(&self) -> &str {
                self.0.as_ref()
            }

        }

        #[cfg(feature = "python")]
        impl_repr!($name);

        #[cfg(feature = "python")]
        impl_eq!($name);

        #[cfg(feature = "python")]
        #[::pyo3::pymethods]
        impl $name {
            #[new]
            fn py_new(value: String) -> Self {
                Self::from(value)
            }

            #[must_use]
            #[getter]
            #[pyo3(name = "is_empty")]
            /// Check if the secret is an empty value
            pub fn py_is_empty(&self) -> bool {
                self.is_empty()
            }

            #[must_use]
            #[getter]
            #[pyo3(name = "secret")]
            /// Get the inner secret contents, which removes the protection against accidentally exposing the value.
            pub fn py_secret(&self) -> &str {
                self.secret()
            }
        }
    }
}

make_secret_string!(
    /// An [OAuth 2.0 refresh token][https://oauth.net/2/refresh-tokens/] that is used to obtain a new [`SecretAccessToken`].
    SecretRefreshToken
);

make_secret_string!(
    /// An [OAuth 2.0 access token][https://oauth.net/2/access-tokens/] that is used to authenticate requests to the QCS API as a `Bearer` token.
    SecretAccessToken
);

make_secret_string!(
    /// The [OAuth2 Client Credentials](https://oauth.net/2/grant-types/client-credentials/) secret.
    ClientSecret
);

#[cfg(test)]
mod test {
    use super::*;

    make_secret_string!(TestSecret);

    #[test]
    fn test_secret_string_serialization() {
        const SECRET_VALUE: &str = "my_secret_value";
        const SECRET_VALUE_JSON: &str = "\"my_secret_value\"";

        // Test that the secret string is a plain JSON string
        assert_eq!(
            serde_json::to_value(TestSecret::from(SECRET_VALUE)).unwrap(),
            serde_json::Value::String(SECRET_VALUE.to_string()),
        );

        let test_secret: TestSecret = serde_json::from_str(SECRET_VALUE_JSON).unwrap();
        assert_eq!(test_secret.secret(), SECRET_VALUE);

        assert_eq!(
            serde_json::to_string(&test_secret).unwrap(),
            SECRET_VALUE_JSON
        );
    }

    #[test]
    fn test_secret_string_debug_does_not_leak() {
        const SECRET_VALUE: &str = "my_secret_value";
        let test_secret = TestSecret::from(SECRET_VALUE);

        let debug_content = format!("{test_secret:?}");

        assert_eq!(
            debug_content,
            format!("TestSecret(<REDACTED len: {}>)", SECRET_VALUE.len())
        );
    }
}
