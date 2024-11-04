use crate::{evm::utils::get_rpc_service, IUniswapV3Factory, V3_FACTORY};
use alloy::{
    primitives::{aliases::U24, Address},
    providers::ProviderBuilder,
    transports::icp::IcpConfig,
};

pub async fn get_pool(token_a: Address, token_b: Address, fee: U24) -> Result<Address, String> {
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    let v3_factory = IUniswapV3Factory::new(V3_FACTORY, provider.clone());

    match v3_factory.getPool(token_a, token_b, fee).call().await {
        Ok(res) => Ok(res.pool),
        Err(e) => Err(e.to_string()),
    }
}
