use crate::{
    auth::only_owner,
    evm::{erc_20, uniswap_v3_factory},
    log::{log_error, log_success, LogItemEvent},
    run::first_run,
    STATE,
};
use std::time::Duration;

pub const FIRST_RUN_DELAY: u64 = 30; // Seconds

#[ic_cdk::update]
async fn start() -> Result<String, String> {
    only_owner()?;

    // Start only if not already running
    STATE.with_borrow_mut(|state| {
        if state.timer_id.is_some() {
            return Err("Already started".to_string());
        }
        Ok(())
    })?;

    let (token_in, token_out, fee) =
        STATE.with_borrow(|state| (state.token_in_address, state.token_out_address, state.fee));

    // Allow the uniswap router contract to spend the base token
    match erc_20::approve(token_in).await {
        Ok(hash) => log_success(LogItemEvent::Approve, Some(hash)),
        Err(msg) => {
            log_error(LogItemEvent::Approve, Some(msg));
            return Err("Start failed, could not approve spending".to_string());
        }
    }

    // Get and save the pool address for the given tokens and fee
    match uniswap_v3_factory::get_pool(token_in, token_out, fee).await {
        Ok(pool_address) => {
            STATE.with_borrow_mut(|state| {
                state.uniswap_v3_pool_address = Some(pool_address);
            });
            log_success(
                LogItemEvent::SavePoolAddress,
                Some(pool_address.to_string()),
            );
        }
        Err(msg) => {
            log_error(LogItemEvent::SavePoolAddress, Some(msg));
            return Err("Start failed, could not save pool address".to_string());
        }
    }

    // Initiate the first run that in turn will set the timer for the next run
    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer(
            Duration::from_secs(FIRST_RUN_DELAY), // Delay to allow the approve transaction to be confirmed creating more transactions.
            first_run,
        ));
    });

    log_success(LogItemEvent::Start, None);
    Ok("Started".to_string())
}
