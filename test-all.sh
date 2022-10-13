#!/usr/bin/env bash

cargo test
cargo tarpaulin --ignore-tests
cargo clippy -- -D warnings
cargo fmt -- --check
cargo audit
