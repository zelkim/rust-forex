use super::account::Account;
use super::forex::{Currency, Forex};

#[derive(Debug)]
pub struct Bank {
    pub forex: Forex,
    pub annual_interest: f64,
    pub base_currency: Currency,
    pub accounts: Vec<Account>,
}

impl Bank {
    // Start with minimal defaults; builder methods will fill in values.
    pub fn new() -> Self {
        Self {
            forex: Forex::new(),
            annual_interest: 0.05,
            base_currency: Currency {
                code: String::from(""),
                name: String::from(""),
                rate: 0.0,
            },
            accounts: Vec::new(),
        }
    }

    pub fn set_forex(mut self, forex: Forex) -> Self {
        self.forex = forex;
        self
    }

    pub fn set_annual_interest(mut self, rate: f64) -> Self {
        self.annual_interest = rate;
        self
    }

    // Sets base_currency by looking up the code in the forex catalog; if not found, creates a placeholder.
    pub fn set_base_currency(mut self, code: &str) -> Self {
        if let Some(cur) = self
            .forex
            .currencies_detailed()
            .into_iter()
            .find(|c| c.code == code)
        {
            self.base_currency = cur;
        } else {
            self.base_currency = Currency {
                code: code.to_string(),
                name: code.to_string(),
                rate: 1.0,
            };
        }
        self
    }

    // Convenience: Builder that sets all core values at once.
    pub fn build(mut self) -> Self {
        // If base_currency not set, try to use forex base rate if available
        if self.base_currency.code.is_empty() {
            let base_code = self.forex.get_base_rate().to_string();
            if let Some(cur) = self
                .forex
                .currencies_detailed()
                .into_iter()
                .find(|c| c.code == base_code)
            {
                self.base_currency = cur;
            } else if !base_code.is_empty() {
                self.base_currency = Currency {
                    code: base_code.clone(),
                    name: base_code,
                    rate: 1.0,
                };
            }
        }
        self
    }

    // Create a new account attached to this bank's annual interest.
    pub fn create_account(&mut self, name: &str) -> &mut Account {
        let acct = Account::with_interest(name, self.annual_interest);
        self.accounts.push(acct);
        let idx = self.accounts.len() - 1;
        &mut self.accounts[idx]
    }
}
