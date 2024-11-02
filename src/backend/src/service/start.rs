use crate::{auth::only_owner, evm::v3_swap_router, usdc::approve, STATE};
use std::time::Duration;

fn tick() {
    ic_cdk::spawn(async move {
        let res = v3_swap_router::swap().await;
        ic_cdk::println!("{:?}", res);
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

    approve().await?;

    STATE.with_borrow_mut(|state| {
        state.timer_id = Some(ic_cdk_timers::set_timer_interval(
            Duration::from_secs(state.settings.interval),
            tick,
        ));
    });

    Ok("Started".to_string())
}
