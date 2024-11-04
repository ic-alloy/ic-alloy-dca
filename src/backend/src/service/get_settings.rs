use crate::{CanisterSettingsDto, State, STATE};

#[ic_cdk::query]
fn get_settings() -> Result<CanisterSettingsDto, String> {
    STATE.with_borrow(|state| {
        let State {
            owner,
            token_in_address,
            token_in_name,
            token_out_address,
            token_out_name,
            fee,
            amount_in,
            slippage,
            interval,
            ..
        } = state;

        Ok(CanisterSettingsDto {
            owner: owner.clone(),
            token_in_address: format!("{}", token_in_address),
            token_in_name: token_in_name.clone(),
            token_out_address: format!("{}", token_out_address),
            token_out_name: token_out_name.clone(),
            fee: fee.try_into().unwrap(),
            amount_in: amount_in.try_into().unwrap(),
            slippage: slippage.try_into().unwrap(),
            interval: *interval,
        })
    })
}
