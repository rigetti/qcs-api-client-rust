#!/bin/bash

set -euo pipefail

COMMIT_MESSAGE="${1:?"Must specify a commit message"}"
TAGS="${@:2}"
if [ ${#TAGS} -eq 0 ]; then
  echo "Must specify the tags to add and push"
  exit 1
fi
WORKDIR="$(mktemp -d)"

if [[ ! "$WORKDIR" || ! -d "$WORKDIR" ]]; then
  echo "Could not create temp dir"
  exit 1
fi

# deletes the temp directory
function cleanup {
  rm -rf "$WORKDIR"
  echo "Deleted temp working directory $WORKDIR"
}

trap cleanup EXIT

git clone "https://$GITHUB_USER_NAME:$GITHUB_TOKEN@github.com/rigetti/qcs-api-client-rust.git" "$WORKDIR"

PUBLIC_DIRS=("qcs-api-client-common" "qcs-api-client-grpc" "qcs-api-client-openapi/public")
PUBLIC_DIRS_RENAME=("qcs-api-client-common" "qcs-api-client-grpc" "qcs-api-client-openapi")

# Delete all files except the hidden directories (.git/ and .github/).
(cd "$WORKDIR"; git rm -r -- *)

for i in "${!PUBLIC_DIRS[@]}"; do
  public_dir=${PUBLIC_DIRS[$i]}
  public_dir_rename=${PUBLIC_DIRS_RENAME[$i]}
  # Copy all files, including hidden files, from the public directory to the public Github repo.
  cp -rf "$public_dir" "$WORKDIR/$public_dir_rename"
done;

cp .gitignore "$WORKDIR"
cp -R .github "$WORKDIR"

# Note: Everything below here is specifically for $WORKDIR.
cd "$WORKDIR"

# We don't just copy over the source Cargo.toml because it would leak
# information about the non-public crates.
cat << EOF > "Cargo.toml"
[workspace]
members = [
    "qcs-api-client-common",
    "qcs-api-client-grpc",
    "qcs-api-client-openapi",
]
EOF

# We use `ex` here because sed is generally incompatible between gnu/linux
# and macos.
ex '+%s/path = "..\/../path = "../g' -scwq qcs-api-client-openapi/Cargo.toml

# This update and check creates a Cargo.lock from the new Cargo.toml and
# verifies that there are no incompatibilities.
cargo update && cargo check

git add --all

# Print the diff and status in the log to aid debugging.
git --no-pager diff --staged
git status

git commit -m "$COMMIT_MESSAGE"
git push

echo "Adding tags: $TAGS"
for t in $TAGS; do
  git tag $t
done
git push --tags
