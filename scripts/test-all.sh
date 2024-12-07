#!/bin/bash

set -euxo pipefail

FEATURES=$1
CHANNEL=$2

cargo test            --verbose --no-default-features
cargo test  --release --verbose --no-default-features

cargo build --verbose --features "$FEATURES"
cargo test  --verbose --features "$FEATURES"

cargo build --release --verbose --features "$FEATURES"
cargo test  --release --verbose --features "$FEATURES"

