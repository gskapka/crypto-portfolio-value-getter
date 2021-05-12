use crate::lib::{assets::Assets, decrypt_api_key::maybe_decrypt_api_keyfile, rates::ExchangeRate, types::Result};

pub fn get_price_of(assets: &[String], amounts: &[f64], currency: &str) -> Result<String> {
    Ok(
        Assets::from_strings(assets, &maybe_decrypt_api_keyfile("./coinmarketcap-api-key.gpg")?)?
            .get_prices_json(amounts, &ExchangeRate::get(currency)?)?
            .to_string(),
    )
}
