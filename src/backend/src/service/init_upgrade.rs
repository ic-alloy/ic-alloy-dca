use std::time::Duration;

use alloy::{
    hex::FromHex,
    primitives::{aliases::U24, Address, U256},
    signers::Signer,
};

use crate::{evm::utils::create_signer, CanisterSettingsDto, State, STATE};

fn save_settings(settings: CanisterSettingsDto) {
    let CanisterSettingsDto {
        owner,
        base_token_address,
        base_token_name,
        swap_token_address,
        swap_token_name,
        fee,
        amount_in,
        slippage,
        interval,
    } = settings;

    let base_token_address = Address::from_hex(base_token_address).unwrap();
    let swap_token_address = Address::from_hex(swap_token_address).unwrap();

    STATE.with_borrow_mut(|state| {
        *state = State {
            owner,
            base_token_address,
            base_token_name,
            swap_token_address,
            swap_token_name,
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
fn init(settings: CanisterSettingsDto) {
    save_settings(settings);
    init_signer();
}

#[ic_cdk::post_upgrade]
fn post_upgrade(settings: CanisterSettingsDto) {
    save_settings(settings);
    init_signer();
}
