#!/bin/bash

set -euo pipefail

COMMIT_MESSAGE="${1:?"Must specify a commit message"}"
WORKDIR=$(mktemp -d -p /tmp)

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

# Delete all files except the .git/ directory.
git rm -r -- "$WORKDIR/*"
# Copy all files, including hidden files, from the public directory to the public Github repo.
cp -rfT ../public "$WORKDIR"

cd "$WORKDIR"
# We cannot update Github workflows using a personal access token, so ignore changes to the following directory.
git reset -- .github/workflows/publish.yaml
git checkout -- .github/workflows/publish.yaml

git add --all
# Print the changes to be committed to the public Github repo.
git status

git commit -m "$COMMIT_MESSAGE"

