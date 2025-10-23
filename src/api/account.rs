/// Transaction types supported by an Account.
/// - Deposit adds a positive amount
/// - Withdraw records a negative amount (see `create_transaction`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
}

/// Immutable transaction record containing the signed value applied
/// to the account balance.
#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub value: f64,
}

/// Bank account model that keeps a running list of transactions and
/// computes balances and interest forecasts. The annual interest is
/// stored per-account so different accounts can have different rates.
#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub transactions: Vec<Transaction>,
    pub annual_interest: f64,
}

impl Account {
    /// Create a new account with a default annual interest (5%).
    /// Simple constructor analogous to constructors in C/Java.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            transactions: Vec::new(),
            annual_interest: 0.05,
        }
    }

    /// Builder method: set the annual interest rate for this account and
    /// return the updated account for chaining.
    /// Usage: `let acct = Account::new("Alice").with_interest(0.05);`
    pub fn with_interest(mut self, annual_interest: f64) -> Self {
        self.annual_interest = annual_interest;
        self
    }

    /// Append a transaction. The `amount` must be > 0.
    /// - Deposit: the stored value is `+amount`.
    /// - Withdraw: the stored value is `-amount`.
    pub fn create_transaction(&mut self, tx_type: TransactionType, amount: f64) {
        assert!(amount > 0.0, "amount must be > 0");
        assert!(
            tx_type == TransactionType::Withdraw 
            && self.get_balance() >= amount 
            || tx_type == TransactionType::Deposit, 
            "insufficient balance for withdrawal"
        );
        let value = match tx_type {
            TransactionType::Deposit => amount,
            TransactionType::Withdraw => -amount,
        };
        self.transactions.push(Transaction { value });
    }

    /// Compute the current balance as the sum of all transaction values.
    pub fn get_balance(&self) -> f64 {
        self.transactions.iter().map(|t| t.value).sum()
    }

    /// Produce a day-by-day compound interest projection using
    /// Daily Interest = Balance × (Annual Rate / 365).
    /// The balance is incremented each day by that day's interest.
    pub fn get_interest_forecast(&self, days: usize) -> Vec<InterestForecast> {
        let daily_rate = self.annual_interest / 365.0;
        let mut balance = self.get_balance();

        (1..=days)
            .map(|day| {
                let interest = balance * daily_rate;
                balance += interest;
                InterestForecast {
                    day,
                    balance,
                    interest,
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct InterestForecast {
    pub day: usize,
    pub balance: f64,
    pub interest: f64,
}
