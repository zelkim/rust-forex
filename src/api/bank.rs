use crate::api::account::Account;
use crate::api::forex::{Currency, Forex};

/// Bank is the top-level orchestrator that holds:
/// - a Forex calculator and registry
/// - a global annual interest rate
/// - a chosen base currency
/// - a list of accounts
///
/// Builder pattern: methods like `set_forex`, `set_annual_interest`, and
/// `set_base_currency` take and return `Self` so calls can be chained
/// fluently (similar to Java builders). Example:
/// `Bank::new().set_forex(...).set_annual_interest(0.05).build()`.
#[derive(Debug)]
pub struct Bank {
    pub forex: Forex,
    pub annual_interest: f64,
    pub base_currency: Currency,
    pub accounts: Vec<Account>,
}

impl Bank {
    /// Create a bank with default fields; builder methods configure details.
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

    /// Set the Forex instance. Returns `Self` for chaining.
    pub fn set_forex(mut self, forex: Forex) -> Self {
        self.forex = forex;
        self
    }

    /// Set the bank-wide annual interest rate as a fraction (e.g., 0.05 = 5%).
    /// Returns `Self` for chaining.
    pub fn set_annual_interest(mut self, rate: f64) -> Self {
        self.annual_interest = rate;
        self
    }

    /// Choose the base currency by code (e.g., "PHP"). If the code is not
    /// already registered in Forex, a placeholder is created. Returns `Self`.
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

    /// Finalize the builder. If `base_currency` is still empty, attempt to use
    /// the `Forex` base code; otherwise, keep as-is.
    pub fn build(mut self) -> Self {
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

    /// Create and store a new account configured with the bank's
    /// current annual interest rate. Returns a mutable reference so
    /// callers can immediately add transactions.
    pub fn create_account(&mut self, name: &str) -> &mut Account {
        let acct = Account::new(name).with_interest(self.annual_interest);
        self.accounts.push(acct);
        let idx = self.accounts.len() - 1;
        &mut self.accounts[idx]
    }

    /// Find an account by name (mutable). Returns `None` if not found.
    pub fn find_account_mut(&mut self, name: &str) -> Option<&mut Account> {
        self.accounts.iter_mut().find(|a| a.name == name)
    }
}
