use crate::evm::utils::get_signer;

#[ic_cdk::update]
pub async fn get_canister_address() -> Result<String, String> {
    let (_, address) = get_signer();
    Ok(address.to_string())
}
