#!/bin/bash

# Use this script to re-generate the candid interface for the agent canister. 
# Pre-requisites: candid-extractor
# Install it with: `cargo install candid-extractor`

# Exit on errors
set -e

# Change to the script's directory
cd "$(dirname "$0")"

# Build the Wasm target
cargo build --target wasm32-unknown-unknown --release

# Navigate to the build output directory
cd ../target/wasm32-unknown-unknown/release

# Extract candid interface from the wasm file
candid-extractor agent.wasm >../../../src/agent/agent.did