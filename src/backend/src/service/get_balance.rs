use alloy::{primitives::Address, providers::ProviderBuilder, transports::icp::IcpConfig};

use crate::{evm::utils::get_rpc_service, IERC20, STATE};

async fn get_balance(token_address: Address) -> Result<String, String> {
    let address = STATE
        .with_borrow(|state| state.canister_eth_address)
        .unwrap();

    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    let contract = IERC20::new(token_address, provider);

    let result = contract.balanceOf(address).call().await;
    match result {
        Ok(balance) => Ok(balance._0.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[ic_cdk::update]
async fn get_balance_in() -> Result<String, String> {
    let token_in_address = STATE.with_borrow(|state| state.token_in_address);
    get_balance(token_in_address).await
}

#[ic_cdk::update]
async fn get_balance_out() -> Result<String, String> {
    let token_out_address = STATE.with_borrow(|state| state.token_out_address);
    get_balance(token_out_address).await
}
