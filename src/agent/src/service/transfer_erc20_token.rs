use alloy::{
    hex::FromHex,
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

use crate::{
    auth::only_owner,
    evm::utils::{get_rpc_service, get_signer},
    log::{log_error, log_success, LogItemEvent},
    IERC20, STATE,
};

async fn transfer(
    token_address: Address,
    recipient: String,
    amount: u64,
) -> Result<String, String> {
    only_owner()?;

    let recipient = Address::from_hex(recipient).map_err(|_| "Invalid recipient address.")?;
    let amount = U256::from(amount);

    let (signer, canister_address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let contract = IERC20::new(token_address, provider);

    match contract
        .transfer(recipient, amount)
        .from(canister_address)
        .send()
        .await
    {
        Ok(res) => {
            log_success(LogItemEvent::Transfer, Some(format!("{}", res.tx_hash())));
            Ok(format!("{}", res.tx_hash()))
        }
        Err(e) => {
            log_error(LogItemEvent::Transfer, Some(e.to_string()));
            Err(e.to_string())
        }
    }
}

#[ic_cdk::update]
async fn transfer_in_token(recipient: String, amount: u64) -> Result<String, String> {
    let token_in_address = STATE.with_borrow(|state| state.token_in_address);
    transfer(token_in_address, recipient, amount).await
}

#[ic_cdk::update]
async fn transfer_out_token(recipient: String, amount: u64) -> Result<String, String> {
    let token_out_address = STATE.with_borrow(|state| state.token_out_address);
    transfer(token_out_address, recipient, amount).await
}
