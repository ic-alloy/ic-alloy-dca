use std::time::Duration;

use crate::{auth::only_owner, usdc::approve, STATE};

fn tick() {
    ic_cdk::spawn(async move {
        let res = approve().await;
        ic_cdk::println!("{:?}", res);
    });
}

#[ic_cdk::update]
fn start() -> Result<String, String> {
    only_owner()?;

    STATE.with_borrow_mut(|state| {
        if state.timer_id.is_some() {
            return Err("Already started".to_string());
        }
        state.timer_id = Some(ic_cdk_timers::set_timer_interval(
            Duration::from_secs(state.settings.interval),
            tick,
        ));
        Ok("Started".to_string())
    })
}
