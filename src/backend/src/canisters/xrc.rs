// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(Debug, CandidType, Deserialize)]
pub enum AssetClass { Cryptocurrency, FiatCurrency }

#[derive(Debug, CandidType, Deserialize)]
pub struct Asset { pub class: AssetClass, pub symbol: String }

#[derive(Debug, CandidType, Deserialize)]
pub struct GetExchangeRateRequest {
  pub timestamp: Option<u64>,
  pub quote_asset: Asset,
  pub base_asset: Asset,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ExchangeRateMetadata {
  pub decimals: u32,
  pub forex_timestamp: Option<u64>,
  pub quote_asset_num_received_rates: u64,
  pub base_asset_num_received_rates: u64,
  pub base_asset_num_queried_sources: u64,
  pub standard_deviation: u64,
  pub quote_asset_num_queried_sources: u64,
}

#[derive(Debug, CandidType, Deserialize)]
pub struct ExchangeRate {
  pub metadata: ExchangeRateMetadata,
  pub rate: u64,
  pub timestamp: u64,
  pub quote_asset: Asset,
  pub base_asset: Asset,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum ExchangeRateError {
  AnonymousPrincipalNotAllowed,
  CryptoQuoteAssetNotFound,
  FailedToAcceptCycles,
  ForexBaseAssetNotFound,
  CryptoBaseAssetNotFound,
  StablecoinRateTooFewRates,
  ForexAssetsNotFound,
  InconsistentRatesReceived,
  RateLimited,
  StablecoinRateZeroRate,
  Other{ code: u32, description: String },
  ForexInvalidTimestamp,
  NotEnoughCycles,
  ForexQuoteAssetNotFound,
  StablecoinRateNotFound,
  Pending,
}

#[derive(Debug, CandidType, Deserialize)]
pub enum GetExchangeRateResult { Ok(ExchangeRate), Err(ExchangeRateError) }

pub struct Xrc(pub Principal);
impl Xrc {
  pub async fn get_exchange_rate(&self, arg0: GetExchangeRateRequest) -> Result<
    (GetExchangeRateResult,)
  > { ic_cdk::call(self.0, "get_exchange_rate", (arg0,)).await }
}
pub const CANISTER_ID : Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 16, 0, 1, 1, 1]); // uf6dk-hyaaa-aaaaq-qaaaq-cai
pub const xrc : Xrc = Xrc(CANISTER_ID);