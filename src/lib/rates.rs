use crate::lib::types::{ Result, NoneError};

use serde_json::Value as JsonValue;

#[derive(Debug)]
pub struct ExchangeRates {
    pub gbp: f64,
}

impl ExchangeRates {
    pub fn get() -> Result<Self> {
        Self::make_rate_reqwest("https://api.exchangeratesapi.io/latest?base=USD")
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

    #[cfg(test)]
    pub fn new_dummy_rates() -> Self {
        Self { gbp: 0.75 }
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
}
