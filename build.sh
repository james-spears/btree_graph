#!/bin/bash

# Run `cargo clean` to guarantee a start with a fresh build cache.
cargo clean

# Run `cargo test --lib` to run unit tests.
cargo test --lib

# Run `cargo test --test integration` to run integration tests.
cargo test --test integration

# Run `cargo build --release` to build the library if tests pass.
cargo build --release
