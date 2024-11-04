use crate::{
    evm::utils::{get_rpc_service, get_signer},
    IUniswapV3SwapRouter, V3_SWAP_ROUTER,
};
use alloy::{
    network::EthereumWallet,
    primitives::{aliases::U24, Address, U160, U256},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn swap(
    token_in: Address,
    token_out: Address,
    fee: U24,
    amount_in: U256,
    amount_out_minimum: U256,
) -> Result<String, String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let args = IUniswapV3SwapRouter::ExactInputSingleParams {
        tokenIn: token_in,
        tokenOut: token_out,
        fee,
        recipient: address,
        amountIn: amount_in,
        amountOutMinimum: amount_out_minimum,
        sqrtPriceLimitX96: U160::from(0),
    };

    let v3_swap_router = IUniswapV3SwapRouter::new(V3_SWAP_ROUTER, provider.clone());

    match v3_swap_router.exactInputSingle(args).send().await {
        Ok(res) => Ok(format!("{}", res.tx_hash())),
        Err(e) => Err(e.to_string()),
    }
}
