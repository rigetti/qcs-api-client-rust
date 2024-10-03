#!/bin/bash

# This script publishes to GitHub the portions of this repo intended to be open source,
# namely the source of the three public crates.
#
# Usage: ./commit-public-repo-update.sh {GitHub commit message} {space-separated git tags to push}

set -euxo pipefail

COMMIT_MESSAGE="${1:?"Must specify a commit message"}"
TAGS="${*:2}"
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

# Ensure we are on the proper branch - necessary for RC releases.
if [[ -n $CI ]] && [[ $CI_COMMIT_REF_NAME != $CI_DEFAULT_BRANCH ]]; then
  (cd "$WORKDIR"; git checkout "$CI_COMMIT_REF_NAME" || git checkout -b "$CI_COMMIT_REF_NAME")
fi

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
cp README.md "$WORKDIR"
cp LICENSE "$WORKDIR"
cp Cargo.toml "$WORKDIR"

# Note: Everything below here is specifically for $WORKDIR.
cd "$WORKDIR"

# Edit Cargo.toml for the public repo; this requires dasel.
if ! command -v dasel &> /dev/null
then
    echo "dasel could not be found. Please install it from https://daseldocs.tomwright.me/installation"
    exit 1
fi
# Remove all workspace members.
dasel delete -f Cargo.toml "workspace.members"
# restore the public crates only.
for i in "${!PUBLIC_DIRS_RENAME[@]}"; do
  public_dir_rename=${PUBLIC_DIRS_RENAME[$i]}
  dasel put -t string -f Cargo.toml -v "$public_dir_rename" 'workspace.members.append()'
done;

# We use `ex` here because sed works differently between gnu/linux and macOS.
ex '+%s/path = "..\/../path = "../g' -scwq qcs-api-client-openapi/Cargo.toml

# This update and check creates a Cargo.lock from the new Cargo.toml and
# verifies that there are no incompatibilities.
cargo update && cargo check

git add --all
if git --no-pager diff --staged --cached --quiet; then
    echo "No changes to commit"
    exit 0
fi

# Print the diff and status in the log to aid debugging.
git --no-pager diff --staged
git status

git commit -m "$COMMIT_MESSAGE"
git push -u origin HEAD

echo "Adding tags: $TAGS"
for t in $TAGS; do
  git tag "$t"
done
git push --tags
