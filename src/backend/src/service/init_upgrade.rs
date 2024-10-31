use std::time::Duration;

use alloy::signers::Signer;

use crate::{create_signer, CanisterSettingsInput, State, STATE};

fn save_settings(settings: CanisterSettingsInput) {
    STATE.with_borrow_mut(|state| {
        *state = State {
            settings,
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
