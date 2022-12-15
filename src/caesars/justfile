#!/usr/bin/env -S just --justfile

# List available commands.
_default:
    just --list --unsorted

# Setup the repository
setup:
    git cliff --version || cargo install git-cliff
    dprint --version || cargo install dprint

# Format the codebase.
fmt:
    dprint fmt --config configs/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    dprint check --config configs/dprint.json

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: fmt

# Check if the repository comply with the rules and ready to be pushed.
check: fmt-check

# Create a new release. Example `just release v2.2.0`
release version:
    bash scripts/release.sh {{ version }}

