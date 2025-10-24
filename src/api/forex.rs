use std::collections::HashMap;

/// Currency value object used by the Forex catalog.
/// - `code`: short identifier like "USD", "PHP".
/// - `name`: human-friendly full name (e.g., "United States Dollar").
/// - `rate`: price of 1 unit of this currency expressed in the base currency.
#[derive(Debug, Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub rate: f64,
}

/// In-memory Forex calculator and registry of currencies.
/// This module only handles exchange rates and does not interact with accounts.
#[derive(Debug)]
pub struct Forex {
    catalog: HashMap<String, Currency>,
    base_currency: String,
}

impl Forex {
    /// Builder-style API note: Some methods take and return `Self` for chaining
    /// (fluent style similar to Java). In Rust, returning `Self` passes
    /// ownership back to the caller so you can write:
    /// `Forex::new().create_currency(...).set_base_rate("PHP")`.
    pub fn new() -> Self {
        Forex {
            catalog: HashMap::new(),
            base_currency: String::new(),
        }
    }

    /// Builder method: registers a currency with a full name and initial rate.
    /// Returns the updated `Forex` so you can chain more calls.
    pub fn create_currency(mut self, code: &str, name: &str, rate: f64) -> Self {
        let currency = Currency { code: code.to_string(), name: name.to_string(), rate: rate };
        self.catalog.insert(currency.code.clone(), currency);
        self
    }

    /// Update the exchange rate for an existing currency `code`.
    /// - If the currency exists, its rate is updated.
    pub fn set_rate(&mut self, code: &str, rate: f64) {
        if self.base_currency == code {
            return;
        }
        if let Some(curr) = self.catalog.get_mut(code) {
            curr.rate = rate;
        }
    }

    /// Get a reference to the rate for `code` if present.
    pub fn get_rate(&self, code: &str) -> Option<&f64> {
        self.catalog.get(code).map(|c| &c.rate)
    }

    /// Builder method: sets the base currency code for this `Forex` and returns
    /// the updated instance for chaining.
    pub fn set_base_rate(mut self, code: &str) -> Self {
        self.base_currency = code.to_string();
        self
    }

    /// Return the current base currency code (e.g., "PHP").
    pub fn get_base_rate(&self) -> &str {
        &self.base_currency
    }

    /// Return a sorted list of all currencies with their code, name, and rate.
    pub fn currencies_detailed(&self) -> Vec<Currency> {
        let mut list: Vec<Currency> = self
            .catalog
            .values()
            .cloned()
            .collect();
        list.sort_by(|a, b| a.code.cmp(&b.code));
        list
    }
}
