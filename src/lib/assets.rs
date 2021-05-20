use serde_json::{json, Value as JsonValue};

use crate::lib::types::{NoneError, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Assets {
    pub assets: Vec<Asset>,
    pub coinmarketcap_api_key: String,
}

impl Assets {
    pub fn new(assets: Vec<Asset>, coinmarketcap_api_key: &str) -> Self {
        Self {
            assets,
            coinmarketcap_api_key: coinmarketcap_api_key.to_string(),
        }
    }

    pub fn from_strings(strings: &[String], coinmarketcap_api_key: &str) -> Self {
        Assets::new(
            strings.iter().map(|ref s| Asset::new(s)).collect::<Vec<Asset>>(),
            coinmarketcap_api_key,
        )
    }

    fn get_prices(&self, amounts: &[f64]) -> Result<Vec<JsonValue>> {
        self.assets
            .iter()
            .enumerate()
            .map(|(i, asset)| asset.get_price_for_x(amounts[i], &self.coinmarketcap_api_key))
            .collect()
    }

    fn get_value_totals(price_jsons: &[JsonValue]) -> Result<Vec<f64>> {
        price_jsons
            .iter()
            .map(|json| -> Result<f64> {
                json.get("value")
                    .ok_or(NoneError("No `value` field in JSON!"))?
                    .as_f64()
                    .ok_or(NoneError("Could not parse to f64!"))
            })
            .collect::<Result<Vec<f64>>>()
    }

    fn sum_values(price_jsons: &[JsonValue]) -> Result<f64> {
        Ok(Self::get_value_totals(price_jsons)?.iter().sum())
    }

    pub fn get_prices_json(&self, amounts: &[f64]) -> Result<JsonValue> {
        let prices = self.get_prices(amounts)?;
        let sum = Self::sum_values(&prices)?;
        Ok(json!({ "total_value": (sum * 100.0).round() / 100.0, "prices": prices }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Asset(String);

impl Asset {
    fn new(s: &str) -> Self {
        Asset(s.to_string())
    }

    fn to_ticker(&self) -> String {
        self.0.clone().to_uppercase()
    }

    fn get_raw_info_from_coinmarketcap(&self, api_key: &str) -> Result<String> {
        Ok(reqwest::blocking::Client::new()
            .get(&format!(
                "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
                self.to_ticker(),
            ))
            .header("X-CMC_PRO_API_KEY", api_key)
            .send()?
            .text()?)
    }

    fn get_usd_price_from_coinmarketcap(&self, api_key: &str) -> Result<f64> {
        let ticker = self.to_ticker();
        Ok(
            serde_json::from_str::<JsonValue>(&self.get_raw_info_from_coinmarketcap(api_key)?)?
                .get("data")
                .ok_or(NoneError("No `data` field in coinmarketcap info!!"))?
                .get(&ticker)
                .ok_or(NoneError("No ticker field in coinmarketcap info!!"))?
                .get("quote")
                .ok_or(NoneError("No `quote` field in coinmarketcap info!!"))?
                .get("USD")
                .ok_or(NoneError("No `USD` field in coinmarketcap info!!"))?
                .get("price")
                .ok_or(NoneError("No `price` field in coinmarketcap info!!"))?
                .to_string()
                .parse::<f64>()?,
        )
    }

    pub fn get_price_for_x(&self, x: f64, api_key: &str) -> Result<JsonValue> {
        let usd_price = self.get_usd_price_from_coinmarketcap(api_key)?;
        Ok(json!({
            "amount": x,
            "asset": self.to_ticker(),
            "value": (usd_price * x * 100.0).round() / 100.0,
            "unit_price_usd": (usd_price * 100.0 ).round() / 100.0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn get_api_key_env_var() -> Result<String> {
        let s = "API_KEY";
        env::var(s).map_err(|_| format!("No `{}` env-var set!", s).into())
    }

    #[test]
    fn should_get_usd_price_of_btc() {
        let asset = Asset::new("BTC");
        let api_key = get_api_key_env_var().unwrap();
        let result = asset.get_usd_price_from_coinmarketcap(&api_key).unwrap();
        assert!(result > 0.0);
        println!("{}", result);
    }
}
