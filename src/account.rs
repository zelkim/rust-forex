// No external date library needed; we use integer day indices.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub transactions: Vec<Transaction>,
    pub annual_interest: f64, // as a fraction, e.g., 0.05 for 5%
}

impl Account {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            transactions: Vec::new(),
            annual_interest: 0.05,
        }
    }

    pub fn with_interest(name: &str, annual_interest: f64) -> Self {
        Self {
            name: name.to_string(),
            transactions: Vec::new(),
            annual_interest,
        }
    }

    pub fn create_transaction(&mut self, transaction_type: TransactionType, amount: f64) {
        assert!(amount > 0.0, "amount must be > 0");
        let value = match transaction_type {
            TransactionType::Deposit => amount,
            TransactionType::Withdraw => -amount,
        };
        self.transactions.push(Transaction { value });
    }

    pub fn get_balance(&self) -> f64 {
        self.transactions.iter().map(|t| t.value).sum()
    }

    pub fn get_daily_interest(&self) -> f64 {
        let daily_rate = self.annual_interest / 365.0;
        self.get_balance() * daily_rate
    }

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
