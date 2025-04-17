#!/bin/bash

# Builds with flags stolen from: https://github.com/johnthagen/min-sized-rust
# Targets minimal binary size
#
# Some of these flags may break certain features in the future
# Our target binary size is <2.5MB uncompressed, prior to static linking
# Post-UPX we get a size of ~500k as of 2025-04-17
URL="http://localhost:8000/api/agent" \
POLL_INTERVAL_MS=10000 \
RUSTFLAGS="-Zlocation-detail=none" \
cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --profile minify

# UPX modifies it's target binary in place, so we copy here
rm ../target/minify/hermes_min &> /dev/null
cp ../target/minify/hermes ../target/minify/hermes_min

# Ultra-brute saves ~1KB of space over LZMA, which is not worth the
# additional time spent bruteforcing different compression techniques 
upx --lzma ../target/minify/hermes_min
# upx --ultra-brute ../target/minify/hermes_min
