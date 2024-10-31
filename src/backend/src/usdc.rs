use alloy::{
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder},
    transports::icp::IcpConfig,
};

use crate::{get_rpc_service, get_signer, IERC20, MAX_ALLOWANCE, SWAP_ROUTER_2, USDC_ADDRESS};

pub async fn approve() -> Result<String, String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let contract = IERC20::new(USDC_ADDRESS, provider.clone());

    match contract
        .approve(SWAP_ROUTER_2, MAX_ALLOWANCE)
        .from(address)
        .send()
        .await
    {
        Ok(builder) => {
            let node_hash = *builder.tx_hash();
            let tx_response = provider.get_transaction_by_hash(node_hash).await.unwrap();

            match tx_response {
                Some(tx) => Ok(format!("{:?}", tx)),
                None => Err("Could not get transaction.".to_string()),
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
