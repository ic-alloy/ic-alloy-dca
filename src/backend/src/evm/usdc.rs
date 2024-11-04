use crate::{
    evm::utils::{get_rpc_service, get_signer},
    IERC20, MAX_ALLOWANCE, STATE, V3_SWAP_ROUTER,
};
use alloy::{network::EthereumWallet, providers::ProviderBuilder, transports::icp::IcpConfig};

pub async fn approve() -> Result<String, String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let base_token = STATE.with_borrow(|state| state.base_token_address);

    let usdc = IERC20::new(base_token, provider.clone());

    match usdc
        .approve(V3_SWAP_ROUTER, MAX_ALLOWANCE)
        .from(address)
        .send()
        .await
    {
        Ok(res) => {
            ic_cdk::println!("Approve OK, result: {:?}", res);
            Ok(format!("{}", res.tx_hash()))
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
