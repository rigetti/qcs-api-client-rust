#!/bin/bash

set -euo pipefail

ROOT_PATH=$(cd "$(dirname "${BASH_SOURCE:-$0}")/.." && pwd)
echo "ROOT_PATH: $ROOT_PATH"

for VARIANT in public internal; do
  echo "Updating $VARIANT"

  python "$ROOT_PATH/scripts/patch_schema.py" "$VARIANT/schema.yaml"
  docker run --rm -v "$ROOT_PATH/$VARIANT:/src" -v "$ROOT_PATH/custom_templates:/custom_templates" openapitools/openapi-generator-cli:v6.0.0 generate \
    -i /src/schema-patched.yaml \
    -g rust \
    -o /src \
    --skip-validate-spec \
    -t /custom_templates

    pushd "$ROOT_PATH/$VARIANT"
    rm schema-patched.yaml
    cargo fmt
    popd
done
