# qcs-api-client-common

[![crates.io version](https://img.shields.io/crates/v/qcs-api-client-common)][crates.io]
[![crates.io license - Apache-2.0](https://img.shields.io/crates/l/qcs-api-client-common)][crates.io]
[![crates.io recent downloads](https://img.shields.io/crates/dr/qcs-api-client-common)][crates.io]
[![docs.rs badge](https://img.shields.io/docsrs/qcs-api-client-common)][docs.rs]

- [crates.io]
- [docs.rs]

This crate serves as a common implementation detail for other `qcs-api-client` crates:

- [`qcs-api-client-grpc`](https://crates.io/crates/qcs-api-client-grpc)
- [`qcs-api-client-openapi`](https://crates.io/crates/qcs-api-client-openapi)

It currently provides utilities for loading QCS configuration and managing authentication tokens.

See [`ClientConfiguration`][clientconfig] for more.

[clientconfig]: https://docs.rs/qcs-api-client-common/latest/qcs_api_client_common/configuration/struct.ClientConfiguration.html
[crates.io]: https://crates.io/crates/qcs-api-client-common
[docs.rs]: https://docs.rs/qcs-api-client-common
