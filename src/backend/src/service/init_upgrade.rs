use std::time::Duration;

use alloy::{
    hex::FromHex,
    primitives::{aliases::U24, Address, U256},
    signers::Signer,
};
use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{evm::utils::create_signer, State, STATE};

#[derive(Serialize, Deserialize, CandidType)]
pub struct CanisterSettingsInput {
    pub owner: String,
    pub base_token: String,
    pub swap_token: String,
    pub fee: u64,
    pub amount_in: u64,
    pub slippage: u64,
    pub interval: u64,
}

fn save_settings(settings: CanisterSettingsInput) {
    let CanisterSettingsInput {
        owner,
        base_token,
        swap_token,
        fee,
        amount_in,
        slippage,
        interval,
    } = settings;

    let base_token = Address::from_hex(base_token).unwrap();
    let swap_token = Address::from_hex(swap_token).unwrap();

    STATE.with_borrow_mut(|state| {
        *state = State {
            owner,
            base_token,
            swap_token,
            fee: U24::from(fee),
            amount_in: U256::from(amount_in),
            slippage: U256::from(slippage),
            interval,
            ..State::default()
        };
    });
}

fn init_signer() {
    ic_cdk_timers::set_timer(Duration::from_secs(0), || {
        ic_cdk::spawn(async move {
            let signer = create_signer().await;
            let address = signer.address();

            STATE.with_borrow_mut(|state| {
                state.signer = Some(signer);
                state.canister_eth_address = Some(address);
            });

            ic_cdk::println!("Starting DCA canister with address: {}", address);
        });
    });
}

#[ic_cdk::init]
fn init(settings: CanisterSettingsInput) {
    save_settings(settings);
    init_signer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade(settings: CanisterSettingsInput) {
    save_settings(settings);
    init_signer();
}
