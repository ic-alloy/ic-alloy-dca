use std::cell::RefCell;

use candid::CandidType;
use serde::Serialize;

#[derive(Serialize, Clone, CandidType)]
pub enum LogItemEvent {
    Approve,
    Swap,
}

#[derive(Serialize, Clone, CandidType)]
pub struct LogItem {
    pub event: LogItemEvent,
    pub ok: Option<String>,
    pub err: Option<String>,
}

thread_local! {
    pub static LOG: RefCell<Vec<LogItem>> = const { RefCell::new(vec![]) };
}

pub fn log_success(event: LogItemEvent, ok: String) {
    LOG.with_borrow_mut(|log| {
        log.push(LogItem {
            event,
            ok: Some(ok),
            err: None,
        });
    });
}

pub fn log_error(event: LogItemEvent, err: String) {
    LOG.with_borrow_mut(|log| {
        log.push(LogItem {
            event,
            ok: None,
            err: Some(err),
        });
    });
}

pub fn get_log() -> Vec<LogItem> {
    LOG.with_borrow(|log| log.to_vec())
}
