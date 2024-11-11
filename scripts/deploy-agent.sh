#!/bin/bash

# Exit on errors
set -e

# Prompt user for network
read -p "Network? (blank for localhost or '--ic' for mainnet) " network

# Change to the script's directory
cd "$(dirname "$0")"

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
