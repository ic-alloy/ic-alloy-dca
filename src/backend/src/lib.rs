mod auth;
mod canisters;
mod evm;
mod log;
mod service;

use alloy::{
    primitives::{address, aliases::U24, Address, U256},
    signers::icp::IcpSigner,
    sol,
};
use candid::CandidType;
use ic_cdk::export_candid;
use ic_cdk_timers::TimerId;
use log::LogItem;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

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

#[derive(Serialize, Deserialize, CandidType)]
pub struct CanisterSettingsDto {
    pub owner: String,
    pub base_token_address: String,
    pub base_token_name: String,
    pub swap_token_address: String,
    pub swap_token_name: String,
    pub fee: u64,
    pub amount_in: u64,
    pub slippage: u64,
    pub interval: u64,
}

#[derive(Default)]
pub struct State {
    // Settings
    owner: String,
    base_token_address: Address,
    base_token_name: String,
    swap_token_address: Address,
    swap_token_name: String,
    fee: U24,
    amount_in: U256,
    slippage: U256,
    interval: u64,

    // Runtime
    timer_id: Option<TimerId>,
    signer: Option<IcpSigner>,
    canister_eth_address: Option<Address>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

export_candid!();
