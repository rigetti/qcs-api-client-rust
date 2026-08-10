#!/usr/bin/env bash
# Determines if `griffe` and `knope` agree about whether this is a breaking change for Python APIs.
# Exits with status 1 if `griffe` says it's a breaking change, but `knope` doesn't know that.
# Exits with status 0 if they both or only `knope` reports this is a breaking change.
#
# This uses `uv` to run `griffe`, and must be executed from the project's root directory.

set -u

# `griffe` needs to run at the project root.
pushd "$(git rev-parse --show-toplevel)" || exit
trap popd EXIT

# `griffe` wants to run in the root, but `uv` needs `--package` to find `pyproject.toml`.
# This adds both the locations for the Python package at different times in the repo's history.
# `griffe` doesn't mind if the path doesn't exist, but if you compare tags across the merge,
# it won't find the Python package at all, and it'll fail with a obscure error.
uv run \
  --project qcs-api-client-common \
  --group dev -- \
    griffe check \
      --search qcs-api-client-common/python \
      --search qcs-api-client-common \
      qcs_api_client_common
api_break=$?

# Now check if `knope` knows this has "Breaking Changes".
#
# This just looks for a line mentioning what will get added to the project's `CHANGELOG.md`,
# and if it exists, looks for the line `### Breaking Changes`.
# If it finds both, it should be a breaking change for the Python package.
# If it doesn't find the one of those lines, or finds other changes before `Breaking Changes`,
# then either there are no breaking changes, or they aren't breaking changes for the package.
knope --dry-run release | awk -f <(cat <<-'EOF'
  BEGIN { is_breaking = 0; }
  /^Would add the following to .*\/CHANGELOG.md: *$/ { is_target = ($6 ~ /qcs-api-client-common/); }
  /^### Breaking Changes$/ && is_target { is_breaking = 1; }
  END { exit is_breaking; }
EOF
)
marked_break=$?

if [[ $api_break == $marked_break ]]; then
  if [[ $api_break == 0 ]]; then
    echo "griffe and knope agree there are NOT any breaking changes"
  else
    echo "griffe and knope agree there ARE SOME breaking changes"
  fi
  exit 0
elif [[ $api_break == 0 ]] ; then
  # This isn't an error, but it might be a surprise.
  echo "knope knows about breaking changes, but griffe doesn't report breaking changes for the Python API"
  exit 0
else
  echo "griffe says this is a breaking change for the Python API, but knope does not know that!"
  exit 1
fi

