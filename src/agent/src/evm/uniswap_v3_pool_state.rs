use crate::{evm::utils::get_rpc_service, IUniswapV3PoolState};
use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn get_estimate_amount_out(
    pool_address: Address,
    amount_in: U256,
) -> Result<U256, String> {
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    let v3_pool_state = IUniswapV3PoolState::new(pool_address, provider);

    match v3_pool_state.slot0().call().await {
        Ok(res) => {
            let sqrt_price_x96 = U256::from(res.sqrtPriceX96);
            let price_ratio = (sqrt_price_x96 * sqrt_price_x96) >> 192;
            Ok(amount_in * price_ratio)
        }
        Err(e) => Err(e.to_string()),
    }
}
