use crate::canisters::{
    self,
    xrc::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult},
};
use alloy::sol;
use candid::Principal;
use ic_cdk::api::call::{call_with_payment128, CallResult};

sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    ETH_USD_FEED,
    "abi/ETH_USD_FEED.json"
);

#[ic_cdk::update]
async fn get_balance(principal: Option<Principal>) -> Result<String, String> {
    // If no principal is specified in call, attempt to use caller principal
    // let principal = principal.unwrap_or_else(ic_cdk::caller);

    // Setup signer
    // let ecdsa_key_name = get_ecdsa_key_name();
    // let derivation_path = create_derivation_path(&principal);
    // let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, None)
    //     .await
    //     .map_err(|e| e.to_string())?;

    // Setup provider
    // let rpc_service = get_rpc_service();
    // let config = IcpConfig::new(rpc_service);
    // let provider = ProviderBuilder::new().on_icp(config);

    // Get balance for signer address
    // let address = signer.address();
    // let result = provider.get_balance(address).await;

    // let contract = ETH_USD_FEED::new(
    //     address!("694AA1769357215DE4FAC081bf1f309aDC325306"),
    //     provider,
    // );

    let args = GetExchangeRateRequest {
        timestamp: None,
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "BTC".to_string(),
        },
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "EUR".to_string(),
        },
    };

    let result: CallResult<(GetExchangeRateResult,)> = call_with_payment128(
        canisters::xrc::CANISTER_ID,
        "get_exchange_rate",
        (args,),
        1_000_000_000,
    )
    .await;

    match result {
        Ok(rate) => Ok(format!("{:?}", rate.0)),
        Err(e) => Err(format!("{:?}", e)),
    }

    //     let res: CallResult<(MultiGetTransactionCountResult,)> = call_with_payment128(
    //     crate::declarations::evm_rpc::evm_rpc.0,
    //     "",
    //     (
    //         chain_config.rpc_services.clone(),
    //         None::<RpcConfig>,
    //         GetTransactionCountArgs {
    //             address: get_self_eth_address().await,
    //             block: BlockTag::Pending,
    //         },
    //     ),
    //     ETH_DEFAULT_CALL_CYCLES,
    // )
    // .await;

    // let result = contract.latestRoundData().call().await;
    // match result {
    //     Ok(balance) => Ok(balance.answer.to_string()),
    //     Err(e) => Err(e.to_string()),
    // }
}
