use crate::{
    evm::utils::{get_rpc_service, get_signer},
    IERC20, MAX_ALLOWANCE, UNISWAP_V3_SWAP_ROUTER,
};
use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn approve(token: Address) -> Result<String, String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let usdc = IERC20::new(token, provider.clone());

    match usdc
        .approve(UNISWAP_V3_SWAP_ROUTER, MAX_ALLOWANCE)
        .from(address)
        .send()
        .await
    {
        Ok(res) => Ok(format!("{}", res.tx_hash())),
        Err(e) => Err(format!("{:?}", e)),
    }
}
