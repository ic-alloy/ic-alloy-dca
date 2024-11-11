use std::cell::RefCell;

use candid::CandidType;
use serde::Serialize;

#[derive(Serialize, Clone, CandidType, Debug)]
pub enum LogItemEvent {
    Start,
    Stop,
    Approve,
    SavePoolAddress,
    Swap,
    Transfer,
}

#[derive(Serialize, Clone, CandidType, Debug)]
pub struct LogItem {
    pub timestamp: u64,
    pub event: LogItemEvent,
    pub ok: Option<String>,
    pub err: Option<String>,
}

thread_local! {
    pub static LOG: RefCell<Vec<LogItem>> = const { RefCell::new(vec![]) };
}

pub fn log_success(event: LogItemEvent, ok: Option<String>) {
    LOG.with_borrow_mut(|log| {
        let item = LogItem {
            timestamp: ic_cdk::api::time(),
            event,
            ok,
            err: None,
        };
        ic_cdk::println!("{:?}", item);
        log.push(item);
    });
}

pub fn log_error(event: LogItemEvent, err: Option<String>) {
    LOG.with_borrow_mut(|log| {
        let item = LogItem {
            timestamp: ic_cdk::api::time(),
            event,
            ok: None,
            err,
        };
        ic_cdk::println!("{:?}", item);
        log.push(item);
    });
}

pub fn get_log() -> Vec<LogItem> {
    LOG.with_borrow(|log| log.to_vec())
}
