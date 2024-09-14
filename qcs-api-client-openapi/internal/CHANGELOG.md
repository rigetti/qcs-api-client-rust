## 0.10.3-rc.0 (2024-09-13)

### Features

#### add support for new 'post-processing' phase

## 0.10.2 (2024-08-29)

### Fixes

#### add defaults to settings profile applications

## 0.10.1 (2024-08-28)

### Fixes

#### Use correct default API URL when building a ClientConfiguration

## 0.10.0 (2024-08-28)

### Breaking Changes

#### Support the client credentials grant type

## 0.9.7 (2024-07-18)

### Fixes

#### Version number consistency

## 0.9.6 (2024-07-11)

### Fixes

#### common Python action will not test installation for an architecture that is different from the host

## 0.9.5 (2024-07-11)

### Fixes

#### Python workflow accepts GitHub token as an argument.

## 0.9.4 (2024-07-10)

### Fixes

#### jsonwebtoken is now version 9.3.0

## 0.9.3 (2024-07-10)

### Fixes

#### common Python action takes a GitHub token as a parameter

#### common Python action takes a GitHub token as a parameter

## 0.9.2 (2024-07-10)

### Fixes

#### common Python action takes a GitHub token as a parameter

## 0.9.1 (2024-07-03)

### Fixes

#### Add reqwest to workspace dependencies

## 0.9.0 (2024-07-03)

### Breaking Changes

#### Add Python API, the builder setters no longer use the `set_` prefix, rename the `channel` module to `tonic`

## 0.8.18 (2024-06-02)

### Features

#### implement retries on disconnect for methods that are safe/idempotent

## 0.8.17 (2024-05-21)

### Fixes

#### regenerate internal OpenAPI client

## 0.8.16 (2024-05-21)

### Fixes

#### trigger new release

## 0.8.15 (2024-05-09)

### Fixes

#### refresh JWT only when expired, not before every request

## 0.8.14 (2024-04-16)

### Fixes

#### resolve linting errors

## 0.8.13 (2024-03-21)

### Features

#### support http1.1 requests via optional grpc-web feature

## 0.8.12 (2024-02-28)

### Fixes

#### Update ExecuteControllerJobRequest documentation

## 0.8.11 (2024-02-16)

### Features

#### add automatic retry logic to clients

## 0.8.10 (2024-02-12)

### Fixes

#### Release schema updates

## 0.8.9 (2024-01-05)

### Features

#### add ExecutionOptions

## 0.8.8 (2023-12-16)

### Fixes

#### separate configuration of client network OTEL tracing from context propagation

## 0.8.7 (2023-12-05)

### Fixes

#### trigger new release after ci fix

## 0.8.6 (2023-11-28)

### Fixes

#### workspace dependencies

## 0.8.5 (2023-11-25)

### Features

#### Update gRPC schemas

## 0.8.4 (2023-11-15)

### Features

#### regenerate client code with new queue policy type

## 0.8.3 (2023-11-11)

### Fixes

#### trace grpc requests with level info

## 0.8.2

### Fixes

- private type was changed from struct to tuple

## 0.8.1

### Features

- Add `get_channel_with_timeout` and `get_channel_with_endpoint` functions for more fine tune configuration of a channel.

## 0.8.0

### Breaking Changes

- If a settings file is incomplete, defaults are used for missing values.

## 0.7.8

### Features

- Update `DEFAULT_GRPC_API_URL`

## 0.7.7

### Fixes

- bump api client versions

## 0.7.6

### Features

- Environment variable overrides for QVM, QUILC, and the GRPC API URLs are now respected when initializing a default client configuration

## 0.7.5

### Features

- update gRPC Proto Definitions

### Fixes

- fix the GrpcService blanket impl on RefreshService<T>

## 0.7.4

### Fixes

- fail knope PrepareRelease if cargo upgrade and git add fails

## 0.7.3

### Features

- try lowercase http(s?)_proxy variables

## 0.7.2

### Features

- add general tracing support

## 0.7.1

### Fixes

- use rustls instead of native openssl-sys

## 0.7.0

### Breaking Changes

- release proxy clients

## 0.6.1

### Fixes

- fix how channels are parsed

## 0.6.0

### Breaking Changes

- default to 64-bit integers when not specified
- fix how channels are parsed
- The change to the common crate's `Error` enum introduces a generic parameter, making the change backwards incompatible.

### Features

- Support refreshing service tokens

### Fixes

- appease clippy

## 0.5.3

### Features

- support otel tracing

### Fixes

- re-export pbjson_types instead of broken include

## 0.5.2

### Fixes

- bump release version

## 0.5.0

### Breaking Changes

- assume `type: number` is `f64` when `format` is unspecified
- add methods for overriding items set via env
- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- QPU Accessor support
- support loading QVM and quilc URLs from environment variables
- Support QPU access calls
- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- failed ci pipelines due to lack of permissions
- make grpc_api_url optional in settings.toml
- version generated protobuf code in crate
- make Endpoint.address nullable and regen
- include LICENSE
- add top-level README
- use api_url not base_path
- install protoc for CI
- clean up generated code, add example to lib.rs docs
- update public grpc proto
- make RefreshService support tonic requests, minor API improvements
- make schema openapi 3.0 compatible
- Crate repo metadata
- Broken generated code via patch_schema.py

## 0.3.8

### Features

- QPU Accessor support

## 0.3.7

### Features

- support loading QVM and quilc URLs from environment variables

## 0.3.6

### Fixes

- make grpc_api_url optional in settings.toml

## 0.3.5

### Fixes

- version generated protobuf code in crate
- make Endpoint.address nullable and regen

## 0.3.4

### Fixes

- include LICENSE

## 0.3.3

### Features

- Support QPU access calls

## 0.3.2

### Fixes

- add top-level README
- use api_url not base_path

## 0.3.1

### Fixes

- install protoc for CI

## 0.3.0

### Breaking Changes

- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- clean up generated code, add example to lib.rs docs
- update public grpc proto
- make RefreshService support tonic requests, minor API improvements
- make schema openapi 3.0 compatible
- Crate repo metadata
- Broken generated code via patch_schema.py
