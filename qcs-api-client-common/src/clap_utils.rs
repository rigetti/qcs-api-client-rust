//! Utilities for common clap functionality

use clap_stdin::MaybeStdin;
use serde::de::DeserializeOwned;
use std::str::FromStr;

/// A JSON-serializable type that can be parsed as a clap argument or from stdin.
pub type JsonMaybeStdin<T> = MaybeStdin<JsonFromStr<T>>;

/// A JSON-serializable type that can be parsed from a string, for use with [`JsonMaybeStdin`].
#[derive(Debug, Clone)]
pub struct JsonFromStr<T>(T);

impl<T> FromStr for JsonFromStr<T>
where
    T: Clone + DeserializeOwned,
{
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = &mut serde_json::Deserializer::from_str(s);
        let v = serde_path_to_error::deserialize(d).map_err(|e| {
            let full_type_name = std::any::type_name::<T>();
            let simple_type_name = full_type_name.rsplit("::").next().unwrap_or(full_type_name);
            miette::Error::from_err(e).wrap_err(format!("Failed to parse {simple_type_name}"))
        })?;
        Ok(Self(v))
    }
}

impl<T> JsonFromStr<T>
where
    T: Clone + DeserializeOwned,
{
    /// Extract the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}
