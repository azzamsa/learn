#!/bin/bash

# Define an array with the directory names
directories=("meta" "nrot")

# Iterate over the array and run the hurl command for each directory
for dir in "${directories[@]}"; do
	hurl --test --glob "${dir}/**/*.hurl" --variables-file props/local
done
