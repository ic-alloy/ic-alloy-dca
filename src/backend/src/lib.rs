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
use ic_cdk::export_candid;
use ic_cdk_timers::TimerId;
use log::LogItem;
use service::init_upgrade::CanisterSettingsInput;
use std::cell::RefCell;

// pub const BASE_ERC20_ADDRESS: Address = address!("1c7D4B196Cb0C7B01d743Fbc6116a902379C7238");
// pub const SWAP_ERC20_ADDRESS: Address = address!("fff9976782d46cc05630d1f6ebab18b2324d6b14");
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

#[derive(Default)]
pub struct State {
    // Settings
    owner: String,
    base_token: Address,
    swap_token: Address,
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
