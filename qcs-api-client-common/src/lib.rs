// Copyright 2022 Rigetti Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Implementation code common to the QCS OpenAPI and gRPC clients.
//!
//! You probably don't need to use this directly, as the clients should expose anything you might
//! need.
//!
//! # Features
//!
//! - `tracing`: enables `tracing` support in [`ClientConfiguration`].
//! - `tracing-config`: enables [`TracingConfiguration`] support for enabling/disabling traces per-URL.
//!   Requires the `tracing` feature.
//! - `python`: enables Python bindings for the client.
pub mod backoff;
pub mod configuration;
pub use configuration::ClientConfiguration;

#[cfg(feature = "clap")]
pub mod clap_utils;

#[cfg(feature = "tracing-config")]
pub mod tracing_configuration;

#[cfg(feature = "python")]
pub mod errors;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
rigetti_pyo3::create_init_submodule! {
    errors: [ errors::QcsApiClientError ],
    submodules: ["configuration": configuration::py::init_submodule],
}

#[cfg(feature = "python")]
#[pymodule]
#[pyo3(name = "_qcs_api_client_common")]
fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    init_submodule("qcs_api_client_common", m.py(), m)
}

#[cfg(feature = "stubs")]
pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
