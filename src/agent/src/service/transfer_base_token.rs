use alloy::{
    hex::FromHex,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    transports::icp::IcpConfig,
};

use crate::{
    auth::only_owner,
    evm::utils::{get_rpc_service, get_signer},
    log::{log_error, log_success, LogItemEvent},
};

#[ic_cdk::update]
async fn transfer_base_token(recipient: String, amount: u64) -> Result<String, String> {
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

    let tx = TransactionRequest::default()
        .with_from(canister_address)
        .with_to(recipient)
        .with_value(amount);

    match provider.send_transaction(tx.clone()).await {
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
