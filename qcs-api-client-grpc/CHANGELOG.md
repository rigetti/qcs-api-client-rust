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
