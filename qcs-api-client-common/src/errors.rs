//! Exception hierarchy exported to the `qcs_api_client_common` Python package.

use pyo3::exceptions::PyException;
use rigetti_pyo3::{create_exception, exception};

create_exception!(
    qcs_api_client_common,
    QcsApiClientError,
    PyException,
    "Base exception type for errors raised by this package."
);

create_exception!(
    qcs_api_client_common.configuration,
    ConfigurationError,
    QcsApiClientError,
    "Base exception type for configuration errors."
);

exception!(
    crate::configuration::error::LoadError,
    qcs_api_client_common.configuration,
    LoadError,
    ConfigurationError,
    "Errors that can occur when loading a configuration."
);

exception!(
    crate::configuration::error::TokenError,
    qcs_api_client_common.configuration,
    TokenError,
    ConfigurationError,
    "Errors that can occur when managing authorization tokens."
);

exception!(
    crate::configuration::ClientConfigurationBuilderError,
    qcs_api_client_common.configuration,
    ClientConfigurationBuilderError,
    ConfigurationError,
    "Unable to build a configuration due to missing or improper values."
);
