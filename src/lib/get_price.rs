use crate::lib::{assets::Assets, types::Result, rates::ExchangeRate};

// TODO get the rate requested by the user!
pub fn get_price_of(assets: &[String], amounts: &[f64], currency: &str) -> Result<String> {
    Ok(Assets::from_strings(assets)?.get_prices_json(amounts, &ExchangeRate::get(currency)?)?.to_string())
}
