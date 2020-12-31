#!/bin/bash

# A script to run a simplified version of the checks done by CI.
#
# Usage
#
# ```sh
# . ./ci.sh
# ```

echo "Running 'cargo fmt'"
cargo +nightly fmt --all

echo "Running 'cargo clippy'"
cargo +nightly clippy --all --all-features --all-targets

echo "Running 'cargo test'"
TRYBUILD=overwrite cargo +nightly test --all --all-features --exclude expandtest

echo "Running 'cargo doc'"
cargo +nightly doc --no-deps --all --all-features

echo "Running 'expandtest'"
# See also https://docs.rs/macrotest/1/macrotest/#updating-expandedrs
# rm **/*.expanded.rs
cargo +nightly test --manifest-path tests/expand/Cargo.toml
