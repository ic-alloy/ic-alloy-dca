use crate::{
    get_rpc_service, get_signer, IV3SwapRouter, USDC_ADDRESS, V3_SWAP_ROUTER, WETH_ADDRESS,
};
use alloy::{
    network::EthereumWallet,
    primitives::{aliases::U24, U160, U256},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn swap() -> Result<(), String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let args = IV3SwapRouter::ExactInputSingleParams {
        tokenIn: USDC_ADDRESS,
        tokenOut: WETH_ADDRESS,
        fee: U24::from(3000),
        recipient: address,
        amountIn: U256::from(100_000),
        amountOutMinimum: U256::from(0),
        sqrtPriceLimitX96: U160::from(0),
    };

    let v3_swap_router = IV3SwapRouter::new(V3_SWAP_ROUTER, provider.clone());

    match v3_swap_router.exactInputSingle(args).send().await {
        Ok(res) => {
            ic_cdk::println!("{:?}", res);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}
