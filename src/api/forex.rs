use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub rate: f64,
}

#[derive(Debug)]
pub struct Forex {
    catalog: HashMap<String, Currency>,
    base_currency: String,
}

impl Forex {
    pub fn new() -> Self {
        Forex {
            catalog: HashMap::new(),
            base_currency: String::new(),
        }
    }

    // Builder method: create a currency with an explicit full name and rate.
    pub fn create_currency(mut self, code: &str, name: &str, rate: f64) -> Self {
        let currency = Currency { code: code.to_string(), name: name.to_string(), rate: rate };
        self.catalog.insert(currency.code.clone(), currency);
        self
    }

    pub fn set_rate(&mut self, code: &str, rate: f64) {
        let code = code.to_string();
        self.catalog
            .entry(code.clone())
            .or_insert(Currency { code: code.clone(), name: code.clone(), rate: rate });
    }

    pub fn get_rate(&self, code: &str) -> Option<&f64> {
        self.catalog.get(code).map(|c| &c.rate)
    }

    pub fn set_base_rate(mut self, code: &str) -> Self {
        self.base_currency = code.to_string();
        self
    }

    pub fn get_base_rate(&self) -> &str {
        &self.base_currency
    }

    pub fn set_currency_name(&mut self, code: &str, name: &str) {
        self.catalog
            .entry(code.to_string())
            .and_modify(|c| c.name = name.to_string())
            .or_insert(Currency { code: code.to_string(), name: name.to_string(), rate: 0.0 });
    }

    pub fn currencies(&self) -> Vec<String> {
        let mut codes: Vec<String> = self.catalog.keys().cloned().collect();
        codes.sort();
        codes
    }

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
