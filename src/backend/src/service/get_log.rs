use crate::log::{self, LogItem};

#[ic_cdk::query]
pub fn get_log() -> Vec<LogItem> {
    log::get_log()
}
