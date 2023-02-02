# Rust clients for Rigetti APIs

This mono-repo contains three Rust crates:

* [`qcs-api-client-common`](./qcs-api-client-common): provides structures
  common to the gRPC and OpenAPI clients. In particular, it provides a
  `ClientConfiguration` type which is used to configure the clients using
  local QCS settings.
* [`qcs-api-client-grpc`](./qcs-api-client-grpc): provides a gRPC client
  for communicating with gRPC-based QCS APIs.
* [`qcs-api-client-openapi`](./qcs-api-client-openapi): provides a OpenAPI
  client for communicating with OpenAPI-based QCS APIs.
  
  
All crates are Apache-2.0.
