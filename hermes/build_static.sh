#!/bin/bash
# Compared to a dynamically built binary, statically linking massively increases file size.
# Uncompressed we hover at around ~7.0MB, and compressed we hover at around ~2.0MB
URL="http://localhost:8000/api/agent" \
POLL_INTERVAL_MS=10000 \
RUSTFLAGS="-Zlocation-detail=none" \
cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --profile minify \
  --target=x86_64-unknown-linux-musl

rm ../target/x86_64-unknown-linux-musl/minify/hermes_min &> /dev/null
cp ../target/x86_64-unknown-linux-musl/minify/hermes ../target/x86_64-unknown-linux-musl/minify/hermes_min

upx --lzma ../target/x86_64-unknown-linux-musl/minify/hermes_min
