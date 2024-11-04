use crate::{
    evm::utils::{get_rpc_service, get_signer},
    IV3SwapRouter, STATE, V3_SWAP_ROUTER,
};
use alloy::{
    network::EthereumWallet,
    primitives::{U160, U256},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn swap() -> Result<String, String> {
    let (signer, address) = get_signer();
    let wallet = EthereumWallet::from(signer);
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_icp(config);

    let (token_in, token_out, fee, amount_in) = STATE.with_borrow(|state| {
        (
            state.base_token_address,
            state.swap_token_address,
            state.fee,
            state.amount_in,
        )
    });

    let args = IV3SwapRouter::ExactInputSingleParams {
        tokenIn: token_in,
        tokenOut: token_out,
        fee,
        recipient: address,
        amountIn: amount_in,
        amountOutMinimum: U256::from(0),
        sqrtPriceLimitX96: U160::from(0),
    };

    let v3_swap_router = IV3SwapRouter::new(V3_SWAP_ROUTER, provider.clone());

    match v3_swap_router.exactInputSingle(args).send().await {
        Ok(res) => {
            ic_cdk::println!("Swap OK, result: {:?}", res);
            Ok(format!("{}", res.tx_hash()))
        }
        Err(e) => Err(e.to_string()),
    }
}
