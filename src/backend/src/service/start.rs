use crate::{
    auth::only_owner,
    evm::{usdc, v3_swap_router},
    log::{log_error, log_success, LogItemEvent},
    STATE,
};
use std::time::Duration;

pub const FIRST_RUN_DELAY: u64 = 30; // Seconds

fn run() {
    ic_cdk::spawn(async move {
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

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer(
            Duration::from_secs(FIRST_RUN_DELAY),
            first_run,
        ));
    });

    Ok("Started".to_string())
}
