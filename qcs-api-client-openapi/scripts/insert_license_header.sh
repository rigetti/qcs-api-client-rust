#!/usr/bin/env bash

set -euo pipefail

# Get the directory that this script is in
ROOT=$(cd "$(dirname -- "${BASH_SOURCE:-$0}")/.." && pwd)

FILE="$1"
TEMP_FILE="$1.tmp"
HEADER="$ROOT/license_header"

cat "$HEADER" "$FILE" > "$TEMP_FILE"
mv "$TEMP_FILE" "$FILE"