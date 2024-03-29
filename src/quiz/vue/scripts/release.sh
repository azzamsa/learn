#!/usr/bin/env bash

untracked_files=$(git ls-files . --exclude-standard --others)

# check for untracked files
if [ -n "$untracked_files" ]; then
    echo "warn: Please stash or remove the untracked files"
    exit 1
fi

# if tag is not passed
if [ -z "$1" ]; then
    echo "warn: Please provide a tag"
    exit 1
fi

# update the version
# jq update file "in-place" so it can't use pipe directly
_new_version=${1#v} # strip the `v` prefix
# shellcheck disable=SC2005
echo "$(jq --arg new_version "$_new_version" '.version = $new_version' package.json)" > package.json

# update the changelog
git-cliff --tag "$1" --sort newest --config configs/cliff.toml > CHANGELOG.md
# format newly added changelog file
just fmt

git add -A && git commit -m "$1"
git show
git tag -s -a "$1" -m "$1" -m "For details, see the CHANGELOG.md"
git tag -v "$1"
