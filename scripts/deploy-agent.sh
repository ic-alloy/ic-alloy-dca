#!/bin/bash

# Exit on errors
set -e

# Prompt user for network
read -p "Network? (blank for localhost or '--ic' for mainnet) " network

# Change to the script's directory
cd "$(dirname "$0")"

# Build the Wasm target
cargo build --target wasm32-unknown-unknown --release

# Navigate to the build output directory
cd ../target/wasm32-unknown-unknown/release

# Extract candid interface from the wasm file
candid-extractor agent.wasm >../../../src/agent/agent.did

# Add candid metadata to the wasm file and set visibility to public
ic-wasm agent.wasm -o agent.wasm metadata candid:service -f ../../../src/agent/agent.did -v public

# Compress the wasm file
gzip -c agent.wasm >agent.wasm.gz

# Back to root folder
cd ../../../

# Deploy with DFX
dfx deploy agent --argument "(
  record {
    owner = \"$(dfx identity get-principal)\";
    token_in_address = \"0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238\";
    token_in_name = \"USDC\";
    token_out_address = \"0xfff9976782d46cc05630d1f6ebab18b2324d6b14\";
    token_out_name = \"WETH\";
    fee = 3000;
    amount_in = 100000;
    slippage = 5;
    interval = 3600;
  }
)" $network
