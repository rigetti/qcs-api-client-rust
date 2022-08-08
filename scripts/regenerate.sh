#!/bin/bash

set -euo pipefail

ROOT_PATH=$(dirname "$0")/..
echo "ROOT_PATH: $ROOT_PATH"

for VARIANT in public internal; do
  echo "Updating $VARIANT"

  docker run --rm -v "$ROOT_PATH/$VARIANT:/src" -v "$ROOT_PATH/custom_templates:/custom_templates" openapitools/openapi-generator-cli:v6.0.0 generate \
    -i /src/schema.yaml \
    -g rust \
    -o /src \
    --skip-validate-spec \
    -t /custom_templates # Only needed until https://github.com/OpenAPITools/openapi-generator/pull/10778 is merged/released
done
