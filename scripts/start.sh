# Prompt user for network
read -p "Network? (blank for localhost or '--ic' for mainnet) " network

dfx canister call agent start $network
