use serde_json::{
    json,
    Value as JsonValue,
};

use crate::lib::{
    types::{
        Result,
        NoneError,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct Assets(pub Vec<Asset>);

impl Assets {
    pub fn new(assets: Vec<Asset> ) -> Self {
        Self(assets)
    }

    pub fn from_strings(strings: &[String]) -> Result<Self> {
        Ok(Assets::new(
            strings.iter().map(|asset_string| Asset::from_str(&asset_string)).collect::<Result<Vec<Asset>>>()?)
        )
    }

    fn get_prices(&self, amounts: &[f64]) -> Result<Vec<JsonValue>> {
        self
            .0
            .iter()
            .enumerate()
            .map(|(i, asset)| asset.get_price_for_x(amounts[i]))
            .collect()
    }

    fn get_price_totals(price_jsons: &[JsonValue]) -> Result<Vec<f64>> {
        price_jsons
            .iter()
            .map(|json| -> Result<f64> {
                Ok(
                   json
                    .get("total")
                    .ok_or(NoneError("No `result` field in JSON!"))?
                    .as_f64()
                    .ok_or(NoneError("Could not parse to f64!"))?
                )
            })
            .collect::<Result<Vec<f64>>>()

    }

    fn sum_totals(price_jsons: &[JsonValue]) -> Result<f64> {
        Ok(Self::get_price_totals(price_jsons)?.iter().sum())
    }

    pub fn get_prices_json(&self, amounts: &[f64]) -> Result<JsonValue> {
        let prices = self.get_prices(amounts)?;
        let sum = Self::sum_totals(&prices)?;
        Ok(json!({ "grand_total": (sum * 100.0).round() / 100.0, "prices": prices }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Asset {
    BTC,
    ADA,
    ETH,
    //PNT, // TODO ADD this in!
}

impl Asset {
    fn to_ticker(&self) -> &str {
        match self {
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
            Asset::ADA => "ADA",
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
        let suffix = "USD";
        let prefix = "https://api.kraken.com/0/public/Ticker?pair=";
        format!("{}{}{}", prefix, self.to_ticker(), suffix)
    }

    fn make_reqwest(&self, url: &str) -> Result<JsonValue> {
        Ok(serde_json::from_str(&reqwest::blocking::get(url)?.text()?)?)
    }

    fn get_price_from_json_response(&self, json_value: &JsonValue) -> Result<f64> {
        let string_vec: Vec<String> = serde_json::from_str(
            &json_value
                .get("result").ok_or(NoneError("No `result` field in JSON!"))?
                .get(self.to_ticker_in_response()).ok_or(NoneError("No response field in JSON!"))?
                .get("c").ok_or(NoneError("No `c` field in JSON"))?
                .to_string()
        )?;
        let f64_vec = string_vec
            .iter()
            .map(|string| -> Result<f64> { Ok(string.parse::<f64>()?)})
            .collect::<Result<Vec<f64>>>()?;
        Ok(f64_vec[0])
    }

    fn get_price(&self) -> Result<f64> {
        self.make_reqwest(&self.get_api_price_call_url())
            .and_then(|json| self.get_price_from_json_response(&json))
    }

    pub fn get_price_for_x(&self, x: f64) -> Result<JsonValue> {
        let price = self.get_price()?;
        Ok(json!({
            "amount": x,
            "price": price,
            "currency": "USD",
            "asset": self.to_ticker(),
            "total": (price * x * 100.0).round() / 100.0,
        }))
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_ref() {
            "BTC" | "BITCOIN" => Ok(Self::BTC),
            "ADA" | "CARDANO" => Ok(Self::ADA),
            "ETH" | "ETHEREUM" => Ok(Self::ETH),
            _ => Err(format!("Unrecognized asset: {}", s).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_price_of_btc() {
        let asset = Asset::BTC;
        let result = asset.get_price().unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_ada() {
        let asset = Asset::ADA;
        let result = asset.get_price().unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_eth() {
        let asset = Asset::ETH;
        let result = asset.get_price().unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_price_of_x_eth() {
        let x:f64 = 3.14;
        let asset = Asset::ETH;
        let result = asset.get_price_for_x(x);
        assert!(result.is_ok());
        println!("{}", result.unwrap().to_string());
    }

    #[test]
    fn should_get_asset_from_str() {
        let string = "eth";
        let result = Asset::from_str(string).unwrap();
        assert_eq!(result, Asset::ETH);
    }
}
