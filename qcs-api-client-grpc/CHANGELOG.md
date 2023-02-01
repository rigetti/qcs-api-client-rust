## 0.3.0

### Breaking Changes

- add methods for overriding items set via env
- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- support loading QVM and quilc URLs from environment variables
- include server when building gRPC code
- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- make grpc_api_url optional in settings.toml
- version generated protobuf code in crate
- include LICENSE
- add top-level README
- install protoc for CI
- replace service with cloned service before use
- make Future returned by RefreshService Send
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

- include LICENSE

## 0.2.3

### Features

- include server when building gRPC code

### Fixes

- add top-level README

## 0.2.2

### Fixes

- install protoc for CI

## 0.2.1

### Fixes

- replace service with cloned service before use

## 0.2.0

### Breaking Changes

- Complete regeneration of `qcs-api` crate using the latest schema.

### Features

- add authentication refresh to clients
- vend client configuration utilities
- generate gRPC clients

### Fixes

- make Future returned by RefreshService Send
- update public grpc proto
- make RefreshService support tonic requests, minor API improvements
- make schema openapi 3.0 compatible
- Crate repo metadata
- Broken generated code via patch_schema.py
