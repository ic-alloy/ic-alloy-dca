use ic_cdk::api::call::{call_with_payment128, CallResult, RejectionCode};

use crate::{
    auth::only_owner,
    canisters::{
        self,
        xrc::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult},
    },
    evm::{usdc, v3_swap_router},
    log::{log_error, log_success, LogItemEvent},
    STATE,
};
use std::time::Duration;

pub const FIRST_RUN_DELAY: u64 = 30; // Seconds

async fn get_estimate_amount_out() {
    let (base_token_name, swap_token_name) =
        STATE.with_borrow(|state| (state.base_token_name.clone(), state.swap_token_name.clone()));

    let args = GetExchangeRateRequest {
        timestamp: None,
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: base_token_name,
        },
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: swap_token_name,
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
        Ok(result) => {
            ic_cdk::println!("OK, get_exchange_rate: {:?}", result);
        }
        Err(e) => {
            ic_cdk::println!("Error: {:?}", e.1);
        }
    }
}

fn run() {
    ic_cdk::spawn(async move {
        // let args = GetExchangeRateRequest {
        //     timestamp: None,
        //     base_asset: Asset {
        //         class: AssetClass::Cryptocurrency,
        //         symbol: "BTC".to_string(),
        //     },
        //     quote_asset: Asset {
        //         class: AssetClass::Cryptocurrency,
        //         symbol: "EUR".to_string(),
        //     },
        // };
        // let result: CallResult<(GetExchangeRateResult,)> = call_with_payment128(
        //     canisters::xrc::CANISTER_ID,
        //     "get_exchange_rate",
        //     (args,),
        //     1_000_000_000,
        // )
        // .await;

        match v3_swap_router::swap().await {
            Ok(hash) => log_success(LogItemEvent::Swap, hash),
            Err(msg) => log_error(LogItemEvent::Swap, msg),
        }
    });
}

fn first_run() {
    run();

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer_interval(
            Duration::from_secs(state.interval),
            run,
        ));
    });
}

fn approve() {
    ic_cdk::spawn(async move {
        match usdc::approve().await {
            Ok(hash) => log_success(LogItemEvent::Approve, hash),
            Err(msg) => log_error(LogItemEvent::Approve, msg),
        }
    });
}

#[ic_cdk::update]
async fn start() -> Result<String, String> {
    only_owner()?;

    STATE.with_borrow_mut(|state| {
        if state.timer_id.is_some() {
            return Err("Already started".to_string());
        }
        Ok(())
    })?;

    approve();
    get_estimate_amount_out().await;

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer(
            Duration::from_secs(FIRST_RUN_DELAY),
            first_run,
        ));
    });

    Ok("Started".to_string())
}
