#!/bin/bash

# Prompt user for network
read -p "Network? (blank for localhost or '--ic' for mainnet) " network

# Prompt for recipient address
read -p "Recipient? " recipient

# Prompt for transfer amount
read -p "Amount? " amount

echo -e "\nTransferring $amount to $recipient...\n"

# Call the canister with the specified recipient and amount
dfx canister call agent transfer_out_token "(\"$recipient\", $amount)" $network
