use crate::{CanisterSettingsDto, State, STATE};

#[ic_cdk::query]
fn get_settings() -> Result<CanisterSettingsDto, String> {
    STATE.with_borrow(|state| {
        let State {
            owner,
            base_token_address,
            base_token_name,
            swap_token_address,
            swap_token_name,
            fee,
            amount_in,
            slippage,
            interval,
            ..
        } = state;

        Ok(CanisterSettingsDto {
            owner: owner.clone(),
            base_token_address: format!("{}", base_token_address),
            base_token_name: base_token_name.clone(),
            swap_token_address: format!("{}", swap_token_address),
            swap_token_name: swap_token_name.clone(),
            fee: fee.try_into().unwrap(),
            amount_in: amount_in.try_into().unwrap(),
            slippage: slippage.try_into().unwrap(),
            interval: *interval,
        })
    })
}
