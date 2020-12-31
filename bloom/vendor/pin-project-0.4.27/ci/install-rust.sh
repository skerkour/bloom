#!/bin/bash

set -euo pipefail

toolchain="${1:-nightly}"

rustup set profile minimal
rustup update "${toolchain}" --no-self-update
rustup default "${toolchain}"

rustup -V
rustc -V
cargo -V
