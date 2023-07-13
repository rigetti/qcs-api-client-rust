## 0.7.1

### Features

- Add `get_channel_with_timeout` and `get_channel_with_endpoint` functions for more fine tune configuration of a channel.

## 0.7.0

### Breaking Changes

- If a settings file is incomplete, defaults are used for missing values.

## 0.6.8

### Features

- Update `DEFAULT_GRPC_API_URL`

## 0.6.7

### Fixes

- bump api client versions

## 0.6.6

### Features

- Environment variable overrides for QVM, QUILC, and the GRPC API URLs are now respected when initializing a default client configuration

## 0.6.5

### Features

- update gRPC Proto Definitions

### Fixes

- fix the GrpcService blanket impl on RefreshService<T>

## 0.6.4

### Fixes

- fail knope PrepareRelease if cargo upgrade and git add fails

## 0.6.3

### Features

- try lowercase http(s?)_proxy variables

## 0.6.2

### Features

- add general tracing support

## 0.6.1

### Fixes

- use rustls instead of native openssl-sys

## 0.6.0

### Breaking Changes

- release proxy clients

## 0.5.0

### Breaking Changes

- The change to the common crate's `Error` enum introduces a generic parameter, making the change backwards incompatible.

### Features

- Support refreshing service tokens

### Fixes

- appease clippy

## 0.4.3

### Features

- support otel tracing

### Fixes

- re-export pbjson_types instead of broken include

## 0.4.2

### Fixes

- bump release version

## 0.4.0

### Breaking Changes

- add methods for overriding items set via env
- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- support loading QVM and quilc URLs from environment variables
- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

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

## 0.2.7

### Features

- support loading QVM and quilc URLs from environment variables

## 0.2.6

### Fixes

- make grpc_api_url optional in settings.toml

## 0.2.5

### Fixes

- version generated protobuf code in crate

## 0.2.4

### Fixes

- use rustls rather than openssl-sys

## 0.2.3

### Fixes

- include LICENSE

## 0.2.2

### Fixes

- add top-level README

## 0.2.1

### Fixes

- install protoc for CI

## 0.2.0

### Breaking Changes

- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- update public grpc proto
- make RefreshService support tonic requests, minor API improvements
- make schema openapi 3.0 compatible
- Crate repo metadata
- Broken generated code via patch_schema.py
