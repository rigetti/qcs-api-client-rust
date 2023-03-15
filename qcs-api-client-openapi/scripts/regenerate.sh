#!/bin/bash

set -euo pipefail

ROOT_PATH=$(cd "$(dirname "${BASH_SOURCE:-$0}")/.." && pwd)
echo "ROOT_PATH: $ROOT_PATH"

for VARIANT in public internal; do
    echo "Updating $VARIANT"

    python3 "$ROOT_PATH/scripts/patch_schema.py" "$VARIANT/schema.yaml"
    docker run --rm -v "$ROOT_PATH/$VARIANT:/src" -v "$ROOT_PATH/custom_templates:/custom_templates" openapitools/openapi-generator-cli:v6.0.0 generate \
        -i /src/schema-patched.yaml \
        --additional-properties=bestFitInt=true \
        -g rust \
        -o /src \
        -t /custom_templates

    if [ "$VARIANT" = "public" ]; then
        find "$ROOT_PATH/$VARIANT/src" -type f -name "*.rs" -execdir bash "$ROOT_PATH/scripts/insert_license_header.sh" "{}" \;
    elif [ "$VARIANT" = "internal" ]; then
        # Use Vim EX mode to edit the file, avoiding cross-platform issues with `sed -i`
        find "$ROOT_PATH/$VARIANT/src" -type f -name "*.rs" -execdir ex '+%s/qcs_api_client_openapi/qcs_api_client_openapi_internal/g' -scwq "{}" \;
    fi

    pushd "$ROOT_PATH/$VARIANT"
    rm schema-patched.yaml
    cargo fmt
    popd
done

"${ROOT_PATH}/scripts/patch_generated_code.sh"
cargo make clippy
