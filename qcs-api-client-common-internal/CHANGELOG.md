## 0.5.3

### Fixes

- fail knope PrepareRelease if cargo upgrade and git add fails

## 0.5.2

### Features

- try lowercase http(s?)_proxy variables

## 0.5.1

### Features

- add general tracing support

## 0.5.0

### Breaking Changes

- release proxy clients
- The change to the common crate's `Error` enum introduces a generic parameter, making the change backwards incompatible.
- add methods for overriding items set via env
- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- Support refreshing service tokens
- support otel tracing
- support loading QVM and quilc URLs from environment variables
- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- use rustls instead of native openssl-sys
- appease clippy
- re-export pbjson_types instead of broken include
- bump release version
- failed ci pipelines due to lack of permissions
- make grpc_api_url optional in settings.toml
- version generated protobuf code in crate
- use rustls rather than openssl-sys
- include LICENSE
- add top-level README
- install protoc for CI
- update public grpc proto
- make RefreshService support tonic requests, minor API improvements
- make schema openapi 3.0 compatible
- Crate repo metadata
- Broken generated code via patch_schema.py
