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

## 0.2.1

### Fixes

- Crate repo metadata

## 0.2.0

### Breaking Changes

- Complete regeneration of `qcs-api` crate using the latest schema.

### Fixes

- Broken generated code via patch_schema.py

## 0.1.0

Release from previous repo.
