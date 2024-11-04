use std::cell::RefCell;

use candid::CandidType;
use serde::Serialize;

#[derive(Serialize, Clone, CandidType, Debug)]
pub enum LogItemEvent {
    Start,
    Stop,
    Approve,
    PoolAddress,
    Swap,
}

#[derive(Serialize, Clone, CandidType, Debug)]
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
        let item = LogItem {
            event,
            ok: Some(ok),
            err: None,
        };
        ic_cdk::println!("{:?}", item);
        log.push(item);
    });
}

pub fn log_error(event: LogItemEvent, err: String) {
    LOG.with_borrow_mut(|log| {
        let item = LogItem {
            event,
            ok: None,
            err: Some(err),
        };
        ic_cdk::println!("{:?}", item);
        log.push(item);
    });
}

pub fn get_log() -> Vec<LogItem> {
    LOG.with_borrow(|log| log.to_vec())
}
