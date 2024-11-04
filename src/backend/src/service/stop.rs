use crate::{auth::only_owner, log::log_success, STATE};

#[ic_cdk::update]
fn stop() -> Result<String, String> {
    only_owner()?;

    STATE.with_borrow_mut(|state| {
        if let Some(timer_id) = state.timer_id {
            ic_cdk_timers::clear_timer(timer_id);
            state.timer_id = None;
            log_success(crate::log::LogItemEvent::Stop, None);
            return Ok("Stopped".to_string());
        }
        Err("Cannot stop what is not started".to_string())
    })
}
