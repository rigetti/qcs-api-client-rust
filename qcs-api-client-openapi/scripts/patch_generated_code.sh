#!/usr/bin/env bash

set -eu

ROOT="$(realpath -- "$(dirname -- "${0}")/..")"

sed_replace() {
    replace_str="${1}"
    file="${2}"
    if [ "${file:0:1}" != "/" ]; then
        file="${ROOT}/${file}"
    fi

    kernel="$(uname -s)"
    case "${kernel}" in
        Linux) sed -i -e "${replace_str}" "${file}" ;;
        Darwin) sed -i '' -e "${replace_str}" "${file}" ;;
        *)
            echo "This script does not support ${kernel}"
            exit 1
            ;;
    esac
}

# Channels uses a tag, but the templates only support untagged enums
sed_replace "s|untagged|tag = \"_type\"|" "internal/src/models/channels.rs"

# The template was modified to use the correct tag name (e.g. "CWChannel") instead of the generator provided one
# ("CwChannel"). This forced the enum variant names to use the tag name though, which is a breaking change.
# This replaces the variant name with the name of the contained struct, which restores it to how it was previously.
sed_replace "s|^\([[:space:]]*\)[[:alnum:]]*(crate::models::\([[:alnum:]]*\)),|\1\2(crate::models::\2),|g" "internal/src/models/channels.rs"
