use crate::{create_derivation_path, get_ecdsa_key_name, get_rpc_service};
use alloy::{
    providers::{Provider, ProviderBuilder},
    signers::{icp::IcpSigner, Signer},
    transports::icp::IcpConfig,
};
use candid::Principal;

#[ic_cdk::update]
async fn get_balance(principal: Option<Principal>) -> Result<String, String> {
    // If no principal is specified in call, attempt to use caller principal
    let principal = principal.unwrap_or_else(ic_cdk::caller);

    // Setup signer
    let ecdsa_key_name = get_ecdsa_key_name();
    let derivation_path = create_derivation_path(&principal);
    let signer = IcpSigner::new(derivation_path, &ecdsa_key_name, None)
        .await
        .map_err(|e| e.to_string())?;

    // Setup provider
    let rpc_service = get_rpc_service();
    let config = IcpConfig::new(rpc_service);
    let provider = ProviderBuilder::new().on_icp(config);

    // Get balance for signer address
    let address = signer.address();
    let result = provider.get_balance(address).await;

    match result {
        Ok(balance) => Ok(balance.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
