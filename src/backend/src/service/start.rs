use alloy::primitives::U256;

use crate::{
    auth::only_owner,
    evm::{
        uniswap_v3_factory, uniswap_v3_pool_state::get_estimate_amount_out, uniswap_v3_swap_router,
        usdc,
    },
    log::{log_error, log_success, LogItemEvent},
    STATE,
};
use std::time::Duration;

pub const FIRST_RUN_DELAY: u64 = 30; // Seconds

fn run() {
    ic_cdk::spawn(async move {
        let (token_in, token_out, fee, amount_in, pool_address, slippage) =
            STATE.with_borrow(|state| {
                (
                    state.base_token_address,
                    state.swap_token_address,
                    state.fee,
                    state.amount_in,
                    state.uniswap_v3_pool_address,
                    state.slippage,
                )
            });

        let amount_out_minimum =
            match get_estimate_amount_out(pool_address.unwrap(), amount_in).await {
                Ok(amount) => amount * (U256::from(100) - slippage) / U256::from(100),
                Err(msg) => {
                    log_error(LogItemEvent::Swap, msg);
                    return;
                }
            };

        match uniswap_v3_swap_router::swap(token_in, token_out, fee, amount_in, amount_out_minimum)
            .await
        {
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

fn approve_spending() {
    ic_cdk::spawn(async move {
        let base_token_address = STATE.with_borrow(|state| state.base_token_address);

        match usdc::approve(base_token_address).await {
            Ok(hash) => log_success(LogItemEvent::Approve, hash),
            Err(msg) => log_error(LogItemEvent::Approve, msg),
        }
    });
}

fn get_pool_address() {
    ic_cdk::spawn(async move {
        let (base_token_address, swap_token_address, fee) = STATE.with_borrow(|state| {
            (
                state.base_token_address,
                state.swap_token_address,
                state.fee,
            )
        });

        match uniswap_v3_factory::get_pool(base_token_address, swap_token_address, fee).await {
            Ok(pool_address) => {
                STATE.with_borrow_mut(|state| {
                    state.uniswap_v3_pool_address = Some(pool_address);
                });
                log_success(LogItemEvent::PoolAddress, pool_address.to_string());
            }
            Err(msg) => log_error(LogItemEvent::PoolAddress, msg),
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

    approve_spending();
    get_pool_address();

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer(
            Duration::from_secs(FIRST_RUN_DELAY),
            first_run,
        ));
    });

    Ok("Started".to_string())
}
