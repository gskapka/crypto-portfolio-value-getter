use crate::lib::{assets::Assets, types::Result};

pub fn get_price_of(assets: &[String], amounts: &[f64]) -> Result<String> {
    Ok(Assets::from_strings(assets)?.get_prices_json(amounts)?.to_string())
}
