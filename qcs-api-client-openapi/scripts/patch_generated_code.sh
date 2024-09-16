#!/usr/bin/env bash

set -eu

ROOT="$(realpath -- "$(dirname -- "${0}")/..")"

if ! hash sed 2>/dev/null; then
    echo "This script requires 'sed' to be installed."
    exit 1
fi

function is_gnu_sed() {
    # Non-GNU `sed` doesn't have a way to check the version...so the
    # `--version` command simply fails.
    sed --version >/dev/null 2>&1
}

sed_replace() {
    replace_str="${1}"
    file="${2}"
    if [ "${file:0:1}" != "/" ]; then
        file="${ROOT}/${file}"
    fi

    if is_gnu_sed; then
        sed -i -e "${replace_str}" "${file}"
    else
        sed -i '' -e "${replace_str}" "${file}"
    fi
}

# Channels uses a tag, but the templates only support untagged enums
sed_replace "s|untagged|tag = \"_type\"|" "internal/src/models/channels.rs"

# The template was modified to use the correct tag name (e.g. "CWChannel") instead of the generator provided one
# ("CwChannel"). This forced the enum variant names to use the tag name though, which is a breaking change.
# This replaces the variant name with the name of the contained struct, which restores it to how it was previously.
sed_replace "s|^\([[:space:]]*\)[[:alnum:]]*(crate::models::\([[:alnum:]]*\)),|\1\2(crate::models::\2),|g" "internal/src/models/channels.rs"

# Channels have serialization manually implemented until serde supports having an untagged fallback variant
sed_replace "/#\[serde(.*)\]/d" "internal/src/models/channels.rs"
sed_replace "s|use serde::{Deserialize, Serialize};|mod manual_serde;|g" "internal/src/models/channels.rs"
sed_replace "s|, Serialize, Deserialize||g" "internal/src/models/channels.rs"
