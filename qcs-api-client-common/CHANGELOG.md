## 0.9.0 (2024-08-28)

### Breaking Changes

#### Support the client credentials grant type

## 0.8.14 (2024-07-18)

### Fixes

#### Version number consistency

## 0.8.13 (2024-07-18)

### Fixes

#### Build script updates linker flags when building with the python feature enabled

## 0.8.12 (2024-07-12)

### Fixes

#### fix upload command

## 0.8.11 (2024-07-11)

### Fixes

#### Remove extra character in build step.

## 0.8.10 (2024-07-11)

### Fixes

#### Sync package versions

## 0.8.7 (2024-07-11)

### Fixes

#### common Python action will not test installation for an architecture that is different from the host

## 0.8.6 (2024-07-11)

### Fixes

#### Remove example that broke release action.

## 0.8.5 (2024-07-11)

### Fixes

#### Python workflow accepts GitHub token as an argument.

## 0.8.4 (2024-07-10)

### Fixes

#### jsonwebtoken is now version 9.3.0

## 0.8.3 (2024-07-10)

### Fixes

#### common Python action takes a GitHub token as a parameter

#### common Python action takes a GitHub token as a parameter

## 0.8.2 (2024-07-10)

### Fixes

#### common Python action takes a GitHub token as a parameter

## 0.8.1 (2024-07-03)

### Fixes

#### Add reqwest to workspace dependencies

## 0.8.0 (2024-07-03)

### Breaking Changes

#### Add Python API, the builder setters no longer use the `set_` prefix, rename the `channel` module to `tonic`

## 0.7.16 (2024-06-02)

### Features

#### implement retries on disconnect for methods that are safe/idempotent

## 0.7.15 (2024-05-21)

### Fixes

#### trigger new release

## 0.7.14 (2024-05-09)

### Fixes

#### refresh JWT only when expired, not before every request

## 0.7.13 (2024-04-16)

### Fixes

#### resolve linting errors

## 0.7.12 (2024-03-21)

### Features

#### support http1.1 requests via optional grpc-web feature

## 0.7.11 (2024-02-28)

### Fixes

#### Update ExecuteControllerJobRequest documentation

## 0.7.10 (2024-02-16)

### Features

#### add automatic retry logic to clients

## 0.7.9 (2024-01-05)

### Features

#### add ExecutionOptions

## 0.7.8 (2023-12-16)

### Fixes

#### separate configuration of client network OTEL tracing from context propagation

## 0.7.7 (2023-12-05)

### Fixes

#### trigger new release after ci fix

## 0.7.6 (2023-11-28)

### Fixes

#### workspace dependencies

## 0.7.5 (2023-11-25)

### Features

#### Update gRPC schemas

## 0.7.4 (2023-11-15)

### Features

#### regenerate client code with new queue policy type

## 0.7.3 (2023-11-11)

### Fixes

#### trace grpc requests with level info

## 0.7.2

### Fixes

- private type was changed from struct to tuple

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
