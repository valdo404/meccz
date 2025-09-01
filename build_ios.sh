#!/bin/bash

# Build script for iOS static library
set -e

echo "Building meccz for iOS..."

# Install iOS targets if not already installed
echo "Checking iOS targets..."
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim

# Create output directory
mkdir -p ios/libs

# Build for iOS device (ARM64)
echo "Building for iOS device (aarch64-apple-ios)..."
cargo build --release --target aarch64-apple-ios --lib

# Build for iOS simulator (Intel)
echo "Building for iOS simulator (x86_64-apple-ios)..."
cargo build --release --target x86_64-apple-ios --lib

# Build for iOS simulator (Apple Silicon)
echo "Building for iOS simulator (aarch64-apple-ios-sim)..."
cargo build --release --target aarch64-apple-ios-sim --lib

# Copy libraries
cp target/aarch64-apple-ios/release/libmeccz.a ios/libs/libmeccz-ios.a
cp target/x86_64-apple-ios/release/libmeccz.a ios/libs/libmeccz-ios-sim.a
cp target/aarch64-apple-ios-sim/release/libmeccz.a ios/libs/libmeccz-ios-sim-arm64.a

# Create universal simulator library
echo "Creating universal simulator library..."
lipo -create \
    ios/libs/libmeccz-ios-sim.a \
    ios/libs/libmeccz-ios-sim-arm64.a \
    -output ios/libs/libmeccz-simulator.a

# Generate C header
echo "Generating C header..."
cbindgen --config cbindgen.toml --crate meccz --output ios/include/meccz.h

echo "iOS build complete!"
echo "Device library: ios/libs/libmeccz-ios.a"
echo "Simulator library: ios/libs/libmeccz-simulator.a"
echo "Headers: ios/include/meccz.h"