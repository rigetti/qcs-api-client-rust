# Rust clients for Rigetti APIs

This mono-repo contains four Rust crates:

* [`qcs-api-client-common`](./qcs-api-client-common): provides structures
  common to the gRPC and OpenAPI clients. In particular, it provides a
  `ClientConfiguration` type which is used to configure the clients using
  local QCS settings.
* [`qcs-api-client-grpc`](./qcs-api-client-grpc): provides a gRPC client
  for communicating with gRPC-based QCS APIs.
* [`qcs-api-client-openapi`](./qcs-api-client-openapi): provides a OpenAPI
  client for communicating with OpenAPI-based QCS APIs.
+ [`qcs-cli`](./qcs-cli): A command-line interface for quick access to all QCS endpoints (both gRPC and OpenAPI), plus additional tools and user secrets/settings management. Note that this crate is not published, only pre-built binaries are available.

  
All crates are Apache-2.0.

### Installing the CLI

Pre-built binaries can be downloaded from https://gitlab.com/rigetti/qcs/clients/qcs-api-client-rust/-/releases.

Public binaries will also be released to https://github.com/rigetti/qcs-api-client-rust/releases.


##### From source

Currently, only possible for internal users. This can take a while.
```sh
cargo install -F internal --git ssh://git@gitlab.com/rigetti/qcs/clients/qcs-api-client-rust.git qcs-cli
```
