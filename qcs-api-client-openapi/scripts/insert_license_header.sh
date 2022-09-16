#!/usr/bin/env bash

set -euo pipefail

# Get the directory that this script is in
ROOT=$(cd "$(dirname -- "${BASH_SOURCE:-$0}")/.." && pwd)

FILE="$1"
TEMP_FILE="$1.tmp"
HEADER="$ROOT/license_header"

HEADER_LINES="$(wc -l < "$HEADER")"

if [ "$(cat "$HEADER")" = "$(head -n"$HEADER_LINES" "$FILE")" ]; then
    echo "license header already in $FILE"
else
    echo "inserting license header into $FILE"
    cat "$HEADER" "$FILE" > "$TEMP_FILE"
    mv "$TEMP_FILE" "$FILE"
fi

