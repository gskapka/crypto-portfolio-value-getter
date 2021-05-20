use crate::lib::{assets::Assets, decrypt_api_key::maybe_decrypt_api_keyfile, types::Result};

pub fn get_price_of(assets: &[String], amounts: &[f64], key_file_path: &str) -> Result<String> {
    Ok(Assets::from_strings(assets, &maybe_decrypt_api_keyfile(key_file_path)?)
        .get_prices_json(amounts)?
        .to_string())
}
