use crate::STATE;
use alloy::{
    primitives::Address,
    signers::icp::IcpSigner,
    transports::icp::{RpcApi, RpcService},
};
// ICP uses different ECDSA key names for mainnet and local
// development.
pub fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "local" => "dfx_test_key".to_string(),
        "ic" => "key_1".to_string(),
        _ => panic!("Unsupported network."),
    }
}

pub fn get_rpc_service() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "https://ic-alloy-evm-rpc-proxy.kristofer-977.workers.dev/eth-sepolia".to_string(),
        headers: None,
    })
}

pub async fn create_signer() -> IcpSigner {
    let ecdsa_key_name = get_ecdsa_key_name();
    IcpSigner::new(vec![], &ecdsa_key_name, None).await.unwrap()
}

pub fn get_signer() -> (IcpSigner, Address) {
    STATE.with_borrow(|state| {
        (
            state.signer.as_ref().unwrap().clone(),
            state.canister_eth_address.unwrap(),
        )
    })
}
