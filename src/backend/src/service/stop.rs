use crate::{auth::only_owner, STATE};

#[ic_cdk::update]
fn stop() -> Result<String, String> {
    only_owner()?;

    STATE.with_borrow_mut(|state| {
        if let Some(timer_id) = state.timer_id {
            ic_cdk_timers::clear_timer(timer_id);
            state.timer_id = None;
            return Ok("Stopped".to_string());
        }
        Err("Cannot stop what is not started".to_string())
    })
}
