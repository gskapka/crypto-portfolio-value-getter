use crate::lib::types::{ Result, NoneError};

use serde_json::Value as JsonValue;


#[derive(Debug)]
pub enum ExchangeRate {
    GBP(f64),
    USD(f64),
}

impl ExchangeRate {
    pub fn get(currency_symbol: &str) -> Result<Self> {
        match currency_symbol {
            "USD" | "usd" => Ok(Self::USD(1.0)),
            "GBP" | "gpb" => Ok(Self::GBP(ExchangeRates::get()?.gbp)),
            _ => Err(format!("Unsupported currency: {}", currency_symbol).into()),
        }
    }

    pub fn get_rate(&self) -> f64 {
        match self {
            ExchangeRate::GBP(rate) => *rate,
            ExchangeRate::USD(rate) => *rate,
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            ExchangeRate::GBP(_) => "GBP".to_string(),
            ExchangeRate::USD(_) => "USD".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ExchangeRates {
    pub gbp: f64,
}

impl ExchangeRates {
    pub fn get() -> Result<Self> {
        Self::make_rate_reqwest("https://api.ratesapi.io/api/latest?base=USD")
            .and_then(|json| Self::get_gbp_from_json(&json))
            .map(|gbp| Self { gbp })
    }

    fn make_rate_reqwest(url: &str) -> Result<JsonValue> {
        Ok(serde_json::from_str(&reqwest::blocking::get(url)?.text()?)?)
    }

    fn get_gbp_from_json(json: &JsonValue) -> Result<f64> {
        let rates = json.get("rates").ok_or(NoneError("No `rates` in JSON!"))?;
        let maybe_gbp = rates.get("GBP").ok_or(NoneError("No `GBP` in rates json!"))?;
        Ok(maybe_gbp.as_f64().ok_or(NoneError("broke"))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_exchange_rates() {
        let rates = ExchangeRates::get().unwrap();
        assert!(rates.gbp > 0.0);
    }

    #[test]
    fn should_get_gbp_exchange_rate_correctly() {
        let result = ExchangeRate::get("GBP").unwrap().get_rate();
        assert!(result > 0.0);
    }

    #[test]
    fn should_get_usd_exchange_rate_correctly() {
        let result = ExchangeRate::get("usd").unwrap().get_rate();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn should_get_gbp_symbol_correctly() {
        let result = ExchangeRate::get("GBP").unwrap().get_symbol();
        assert_eq!(result, "GBP");
    }

    #[test]
    fn should_get_usd_symbol_correctly() {
        let result = ExchangeRate::get("usd").unwrap().get_symbol();
        assert_eq!(result, "USD");
    }
}
