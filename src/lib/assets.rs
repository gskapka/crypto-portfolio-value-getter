use serde_json::{json, Value as JsonValue};

use crate::lib::{
    rates::ExchangeRates,
    types::{NoneError, Result},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Assets(pub Vec<Asset>);

impl Assets {
    pub fn new(assets: Vec<Asset>) -> Self {
        Self(assets)
    }

    pub fn from_strings(strings: &[String]) -> Result<Self> {
        Ok(Assets::new(
            strings
                .iter()
                .map(|asset_string| Asset::from_str(&asset_string))
                .collect::<Result<Vec<Asset>>>()?,
        ))
    }

    fn get_prices(&self, amounts: &[f64], rates: &ExchangeRates) -> Result<Vec<JsonValue>> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, asset)| asset.get_price_for_x(amounts[i], rates))
            .collect()
    }

    fn get_price_totals(price_jsons: &[JsonValue]) -> Result<Vec<f64>> {
        price_jsons
            .iter()
            .map(|json| -> Result<f64> {
                Ok(json
                    .get("total")
                    .ok_or(NoneError("No `result` field in JSON!"))?
                    .as_f64()
                    .ok_or(NoneError("Could not parse to f64!"))?)
            })
            .collect::<Result<Vec<f64>>>()
    }

    fn sum_totals(price_jsons: &[JsonValue]) -> Result<f64> {
        Ok(Self::get_price_totals(price_jsons)?.iter().sum())
    }

    pub fn get_prices_json(&self, amounts: &[f64], rates: &ExchangeRates) -> Result<JsonValue> {
        let prices = self.get_prices(amounts, rates)?;
        let sum = Self::sum_totals(&prices)?;
        Ok(json!({ "grand_total": (sum * 100.0).round() / 100.0, "prices": prices }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Asset {
    BTC,
    ADA,
    ETH,
    XMR,
    PNT,
}

impl Asset {
    fn to_ticker(&self) -> &str {
        match self {
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
            Asset::ADA => "ADA",
            Asset::PNT => "PNT",
            Asset::XMR => "XMR",
        }
    }

    fn to_ticker_in_response(&self) -> String {
        let suffix = "ZUSD";
        match self {
            Asset::ADA => "ADAUSD".to_string(),
            Asset::BTC => format!("XXBT{}", suffix),
            _ => format!("X{}{}", self.to_ticker(), suffix),
        }
    }

    fn get_api_price_call_url(&self) -> String {
        match self {
            Asset::PNT => "https://api.coingecko.com/api/v3/simple/token_price/ethereum?contract_addresses=0x89Ab32156e46F46D02ade3FEcbe5Fc4243B9AAeD&vs_currencies=btc".to_string(),
            _ => {
                let suffix = "USD";
                let prefix = "https://api.kraken.com/0/public/Ticker?pair=";
                format!("{}{}{}", prefix, self.to_ticker(), suffix)
            }
        }
    }

    fn make_reqwest(&self, url: &str) -> Result<JsonValue> {
        Ok(serde_json::from_str(&reqwest::blocking::get(url)?.text()?)?)
    }

    fn get_price_from_json_response(&self, json_value: &JsonValue, rates: &ExchangeRates) -> Result<f64> {
        match self {
            Asset::PNT => {
                let pnt_price_in_btc = json_value
                    .get("0x89ab32156e46f46d02ade3fecbe5fc4243b9aaed")
                    .ok_or(NoneError("No PNT contract address field in JSON!"))?
                    .get("btc")
                    .ok_or(NoneError("No `btc` field in JSON!"))?
                    .as_f64()
                    .ok_or(NoneError("Could not parse value to f64!"))?;
                let btc_price_in_usd = Asset::from_str("btc")?.get_price(rates)?;
                let pnt_price_in_usd = pnt_price_in_btc * btc_price_in_usd;
                Ok(pnt_price_in_usd)
            }
            _ => {
                let string_vec: Vec<String> = serde_json::from_str(
                    &json_value
                        .get("result")
                        .ok_or(NoneError("No `result` field in JSON!"))?
                        .get(self.to_ticker_in_response())
                        .ok_or(NoneError("No response field in JSON!"))?
                        .get("c")
                        .ok_or(NoneError("No `c` field in JSON"))?
                        .to_string(),
                )?;
                let f64_vec = string_vec
                    .iter()
                    .map(|string| -> Result<f64> { Ok(string.parse::<f64>()?) })
                    .collect::<Result<Vec<f64>>>()?;
                Ok(f64_vec[0])
            }
        }
    }

    fn get_price(&self, rates: &ExchangeRates) -> Result<f64> {
        self.make_reqwest(&self.get_api_price_call_url())
            .and_then(|json| self.get_price_from_json_response(&json, rates))
            .map(|price| price * rates.gbp)
    }

    pub fn get_price_for_x(&self, x: f64, rates: &ExchangeRates) -> Result<JsonValue> {
        let price = self.get_price(rates)?;
        Ok(json!({
            "amount": x,
            "price": price,
            "currency": "GBP",
            "asset": self.to_ticker(),
            "total": (price * x * 100.0).round() / 100.0,
        }))
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_ref() {
            "XMR" | "MONERO" => Ok(Self::XMR),
            "BTC" | "BITCOIN" => Ok(Self::BTC),
            "ADA" | "CARDANO" => Ok(Self::ADA),
            "PNT" | "PNETWORK" => Ok(Self::PNT),
            "ETH" | "ETHEREUM" => Ok(Self::ETH),
            _ => Err(format!("Unrecognized asset: {}", s).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_price_of_btc() {
        let asset = Asset::BTC;
        let result = asset.get_price(&ExchangeRates::new_dummy_rates()).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_ada() {
        let asset = Asset::ADA;
        let result = asset.get_price(&ExchangeRates::new_dummy_rates()).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_eth() {
        let asset = Asset::ETH;
        let result = asset.get_price(&ExchangeRates::new_dummy_rates()).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_x_eth() {
        #[allow(clippy::approx_constant)]
        let x: f64 = 3.14;
        let asset = Asset::ETH;
        let result = asset.get_price_for_x(x, &ExchangeRates::new_dummy_rates());
        assert!(result.is_ok());
        println!("{}", result.unwrap().to_string());
    }

    #[test]
    fn should_get_asset_from_str() {
        let string = "eth";
        let result = Asset::from_str(string).unwrap();
        assert_eq!(result, Asset::ETH);
    }

    #[test]
    fn should_get_pnt_price() {
        let amount = 1.0;
        let asset = Asset::from_str("pnt").unwrap();
        let result = asset.get_price_for_x(amount, &ExchangeRates::new_dummy_rates()).unwrap();
        println!("result {}", result.to_string());
    }
}
