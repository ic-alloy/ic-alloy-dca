use candid::Principal;

use crate::STATE;

pub fn only_owner() -> Result<(), String> {
    STATE.with_borrow(|state| {
        let owner = Principal::from_text(state.owner.clone()).unwrap();
        if ic_cdk::caller() != owner {
            return Err("Only owner allowed".to_string());
        }
        Ok(())
    })
}
