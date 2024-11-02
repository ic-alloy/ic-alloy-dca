mod auth;
mod canisters;
mod evm;
mod service;
mod usdc;

use alloy::{
    primitives::{address, Address, U256},
    signers::icp::IcpSigner,
    sol,
    transports::icp::{RpcApi, RpcService},
};
use candid::{CandidType, Nat, Principal};
use ic_cdk::export_candid;
use ic_cdk_timers::TimerId;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::cell::RefCell;

pub const USDC_ADDRESS: Address = address!("1c7D4B196Cb0C7B01d743Fbc6116a902379C7238");
pub const WETH_ADDRESS: Address = address!("fff9976782d46cc05630d1f6ebab18b2324d6b14");
pub const V3_SWAP_ROUTER: Address = address!("3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E");
pub const MAX_ALLOWANCE: U256 = U256::MAX;

sol!(
    #[sol(rpc)]
    "sol/IV3SwapRouter.sol"
);

sol!(
    #[sol(rpc)]
    "sol/IERC20.sol"
);

#[derive(Serialize, Deserialize, CandidType, Default)]
pub struct CanisterSettingsInput {
    owner: String,
    asset: String,
    interval: u64,
    amount: u64,
}

#[derive(Default)]
pub struct State {
    settings: CanisterSettingsInput,
    timer_id: Option<TimerId>,
    signer: Option<IcpSigner>,
    canister_eth_address: Option<Address>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    // static SIGNER: RefCell<Option<IcpSigner>> = const { RefCell::new(None) };
}

// ICP uses different ECDSA key names for mainnet and local
// development.
fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "local" => "dfx_test_key".to_string(),
        "ic" => "key_1".to_string(),
        _ => panic!("Unsupported network."),
    }
}

// Modify this function to determine which EVM network canister connects to
fn get_rpc_service() -> RpcService {
    RpcService::Custom(RpcApi {
        url: "https://ic-alloy-evm-rpc-proxy.kristofer-977.workers.dev/eth-sepolia".to_string(),
        headers: None,
    })
}

fn create_derivation_path(principal: &Principal) -> Vec<Vec<u8>> {
    const SCHEMA_V1: u8 = 1;
    [
        ByteBuf::from(vec![SCHEMA_V1]),
        ByteBuf::from(principal.as_slice().to_vec()),
    ]
    .iter()
    .map(|x| x.to_vec())
    .collect()
}

fn auth_guard() -> Result<(), String> {
    match ic_cdk::caller() {
        caller if caller == Principal::anonymous() => {
            Err("Calls with the anonymous principal are not allowed.".to_string())
        }
        _ => Ok(()),
    }
}

async fn create_signer() -> IcpSigner {
    let ecdsa_key_name = get_ecdsa_key_name();
    IcpSigner::new(vec![], &ecdsa_key_name, None).await.unwrap()
}

fn get_signer() -> (IcpSigner, Address) {
    STATE.with_borrow(|state| {
        (
            state.signer.as_ref().unwrap().clone(),
            state.canister_eth_address.unwrap(),
        )
    })
}

export_candid!();
