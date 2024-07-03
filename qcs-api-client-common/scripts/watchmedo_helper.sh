#!/bin/bash
# A helper script intended to be used with watchdog's `watchmedo` command.
# If a Rust file has changed, then the the qcs-api-client-common Python package
# will be rebuilt and installed in the virtual environment before running
# the desired command. See the *-watch tasks in Makefile.toml for usage.
FILE="$1"
COMMAND="$2"

run_command() {
    eval $COMMAND
}

if [[ "$FILE" =~ \.rs$ ]]; then
    echo "Rust file changed, rebuilding project and running Python tests..."
    cargo make install-python-package && run_command
elif [[ "$FILE" =~ \.py$ ]] || [[ "$FILE" =~ \.pyi$ ]]; then
    echo "Python file changed, running tests..."
    run_command
fi
