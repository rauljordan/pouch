#!/bin/bash

set -euo pipefail

export RUSTFLAGS="-D warnings"
export RUSTFMT_CI=1

# Print version information
rustc -Vv
cargo -V

# Build and test main crate
if [ "$FEATURES" == "default" ]; then
    cargo build --locked
    cargo test
else
    cargo build --locked --features=atomic
    cargo test --features=atomic
fi