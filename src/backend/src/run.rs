use crate::{
    evm::{uniswap_v3_pool_state::get_estimate_amount_out, uniswap_v3_swap_router},
    log::{log_error, log_success, LogItemEvent},
    STATE,
};
use alloy::primitives::U256;
use std::time::Duration;

pub fn run() {
    ic_cdk::spawn(async move {
        let (token_in, token_out, fee, amount_in, pool_address, slippage) =
            STATE.with_borrow(|state| {
                (
                    state.token_in_address,
                    state.token_out_address,
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
                    log_error(LogItemEvent::Swap, Some(msg));
                    return;
                }
            };

        match uniswap_v3_swap_router::swap(token_in, token_out, fee, amount_in, amount_out_minimum)
            .await
        {
            Ok(hash) => log_success(LogItemEvent::Swap, Some(hash)),
            Err(msg) => log_error(LogItemEvent::Swap, Some(msg)),
        }
    });
}

pub fn first_run() {
    run();

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer_interval(
            Duration::from_secs(state.interval),
            run,
        ));
    });
}
