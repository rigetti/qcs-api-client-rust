> This document is for internal development purposes only. It should not be published to the public repository.

# Development

## Public vs internal repositories

This repository is manually mirrored via the [./scripts/commit-public-repo-update.sh](./scripts/commit-public-repo-update.sh) to the public repository at https://github.com/rigetti/qcs-api-client-rust. This script purges all source code and documentation intended for internal-only consumption. This includes editing [Cargo.toml](./Cargo.toml) at the root of the repository.

## Managing shared dependencies

The public and internal common, openapi, and grpc repositories share a number dependencies. These dependencies should be defined at the workspace level where possible and crate-level manifests should be updated to point to them as appropriate.
