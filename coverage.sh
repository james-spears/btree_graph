#!/bin/bash

# Run coverage report on the working directory.
cargo kcov -- --verify --include-pattern="$(pwd)" --exclude-pattern=test.rs