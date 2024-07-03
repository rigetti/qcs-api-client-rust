## 0.8.0 (2024-07-03)

### Breaking Changes

#### Complete regeneration of `qcs-api` crate using the latest schema.

#### add methods for overriding items set via env

#### The change to the common crate's `Error` enum introduces a generic parameter, making the change backwards incompatible.

#### release proxy clients

#### If a settings file is incomplete, defaults are used for missing values.

#### Add Python API, the builder setters no longer use the `set_` prefix, rename the `channel` module to `tonic`

### Features

#### generate gRPC clients

#### vend client configuration utilities

#### add authentication refresh to clients

#### support loading QVM and quilc URLs from environment variables

#### support otel tracing

#### Support refreshing service tokens

#### add general tracing support

#### try lowercase http(s?)_proxy variables

#### update gRPC Proto Definitions

#### Environment variable overrides for QVM, QUILC, and the GRPC API URLs are now respected when initializing a default client configuration

#### Update `DEFAULT_GRPC_API_URL`

#### Add `get_channel_with_timeout` and `get_channel_with_endpoint` functions for more fine tune configuration of a channel.

#### regenerate client code with new queue policy type

#### Update gRPC schemas

#### add ExecutionOptions

#### add automatic retry logic to clients

#### support http1.1 requests via optional grpc-web feature

#### implement retries on disconnect for methods that are safe/idempotent

### Fixes

#### Broken generated code via patch_schema.py

#### Crate repo metadata

#### make schema openapi 3.0 compatible

#### make RefreshService support tonic requests, minor API improvements

#### update public grpc proto

#### install protoc for CI

#### add top-level README

#### include LICENSE

#### version generated protobuf code in crate

#### failed ci pipelines due to lack of permissions

#### bump release version

#### re-export pbjson_types instead of broken include

#### appease clippy

#### use rustls instead of native openssl-sys

#### fail knope PrepareRelease if cargo upgrade and git add fails

#### fix the GrpcService blanket impl on RefreshService<T>

#### bump api client versions

#### private type was changed from struct to tuple

#### trace grpc requests with level info

#### workspace dependencies

#### trigger new release after ci fix

#### separate configuration of client network OTEL tracing from context propagation

#### Update ExecuteControllerJobRequest documentation

#### resolve linting errors

#### refresh JWT only when expired, not before every request

#### trigger new release