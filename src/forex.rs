use std::collections::HashMap;

pub struct Forex {
    rates: HashMap<String, f64>,
    base_currency: String,
}

impl Forex {
    pub fn new() -> Self {
        Forex {
            rates: HashMap::new(),
            base_currency: String::new(),
        }
    }
    
    // Builder pattern method to add a currency rate
    pub fn create_rate(mut self, currency: &str, rate: f64) -> Self {
        self.rates.insert(currency.to_string(), rate);
        self
    }

    pub fn set_rate(&mut self, currency: &str, rate: f64) {
        self.rates.insert(currency.to_string(), rate);
    }
    
    pub fn get_rate(&self, currency: &str) -> Option<&f64> {
        self.rates.get(currency)
    }
    
    pub fn set_base_rate(mut self, currency: &str) -> Self {
        self.base_currency = currency.to_string();
        self
    }
    
    pub fn currencies(&self) -> Vec<String> {
        self.rates.keys().cloned().collect()
    }
}
