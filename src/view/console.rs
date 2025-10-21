use std::io::{self, Write};

use crate::api::{account::TransactionType, bank::Bank};

pub struct ConsoleApp {
    pub bank: Bank,
}

impl ConsoleApp {
    pub fn new(bank: Bank) -> Self {
        Self { bank }
    }

    pub fn run(&mut self) {
        loop {
            println!("\nMain Menu\n");
            println!("Select Transaction:");
            println!("[1] Register Account Name");
            println!("[2] Deposit Amount");
            println!("[3] Withdraw Amount");
            println!("[4] Currency Exchange");
            println!("[5] Record Exchange Rates");
            println!("[6] Show Interest Computation");

            let choice = read_usize_prompt("");
            match choice {
                1 => self.menu_register_account(),
                2 => self.menu_deposit(),
                3 => self.menu_withdraw(),
                4 => self.menu_currency_exchange(),
                5 => self.menu_record_exchange_rate(),
                6 => self.menu_show_interest(),
                _ => println!("Invalid option. Please select 1-6."),
            }

            if !ask_yes_no("Back to the Main Menu (Y/N): ") {
                break;
            }
        }
    }

    fn menu_register_account(&mut self) {
        println!("\nRegister Account Name\n");
        println!("Register Account Name");
        let name = read_string_prompt("Account Name: ");
        let _ = self.bank.create_account(&name);
    }

    fn menu_deposit(&mut self) {
        println!("\nDeposit Amount\n");
        let name = read_string_prompt("Account Name: ");
        let currency_code = self.bank.base_currency.code.clone();
    if let Some(acct) = self.bank.find_account_mut(&name) {
            println!("Current Balance: {:.2}", acct.get_balance());
            println!("Currency: {}", currency_code);
            let amount = read_f64_prompt("Deposit Amount: ");
            acct.create_transaction(TransactionType::Deposit, amount);
            println!("Updated Balance: {:.2}", acct.get_balance());
        } else {
            println!("Account not found. Please register first.");
        }
    }

    fn menu_withdraw(&mut self) {
        println!("\nWithdraw Amount\n");
        let name = read_string_prompt("Account Name: ");
        let currency_code = self.bank.base_currency.code.clone();
    if let Some(acct) = self.bank.find_account_mut(&name) {
            println!("Current Balance: {:.2}", acct.get_balance());
            println!("Currency: {}", currency_code);
            let amount = read_f64_prompt("Withdraw Amount: ");
            acct.create_transaction(TransactionType::Withdraw, amount);
            println!("Updated Balance: {:.2}", acct.get_balance());
        } else {
            println!("Account not found. Please register first.");
        }
    }

    fn menu_record_exchange_rate(&mut self) {
        println!("\nRecord Exchange Rate");
        let (codes, names) = currency_menu_lists(&self.bank);
        print_currency_menu(&names);
        let sel = read_usize_prompt("Select Foreign Currency: ");
        if let Some(code) = codes.get(sel.saturating_sub(1)).cloned() {
            let new_rate = read_f64_prompt("Exchange Rate: ");

            let before = self.bank.forex.get_rate(&code).copied();
            self.bank.forex.set_rate(&code, new_rate);
            let after = self.bank.forex.get_rate(&code).copied();
            match (before, after) {
                (Some(old), Some(curr)) if (old - curr).abs() < f64::EPSILON => {
                    println!("Note: Exchange rate for {} was not updated by set_rate.", code);
                }
                _ => println!("Recorded exchange rate for {}.", code),
            }
        } else {
            println!("Invalid selection.");
        }
    }

    fn menu_currency_exchange(&mut self) {
        loop {
            println!("\nForeign Currency Exchange");
            let (codes, names) = currency_menu_lists(&self.bank);
            println!("Source Currency Option:");
            print_currency_menu(&names);
            let src_sel = read_usize_prompt("Source Currency: ");
            if let Some(src) = codes.get(src_sel.saturating_sub(1)).cloned() {
                let amount = read_f64_prompt("Source Amount: ");
                println!("Exchanged Currency Options:");
                print_currency_menu(&names);
                let dst_sel = read_usize_prompt("Exchange Currency: ");
                if let Some(dst) = codes.get(dst_sel.saturating_sub(1)).cloned() {
                    match convert_amount(&self.bank, &src, &dst, amount) {
                        Some(out) => println!("Exchange Amount: {:.2}", out),
                        None => println!("Cannot convert due to missing rates."),
                    }
                } else {
                    println!("Invalid selection.");
                }
            } else {
                println!("Invalid selection.");
            }

            if !ask_yes_no("Convert another currency (Y/N)? ") {
                break;
            }
        }
    }

    fn menu_show_interest(&mut self) {
        println!("\nShow Interest Amount\n");
        let name = read_string_prompt("Account Name: ");
        let currency_code = self.bank.base_currency.code.clone();
        let interest_rate = self.bank.annual_interest;
    if let Some(acct) = self.bank.find_account_mut(&name) {
            println!("Current Balance: {:.2}", acct.get_balance());
            println!("Currency: {}", currency_code);
            println!("Interest Rate: {:.0}%", interest_rate * 100.0);
            let days = read_usize_prompt("Total Number of Days: ");
            let forecast = acct.get_interest_forecast(days);
            println!("Day \t| Interest \t| Balance |");
            for f in forecast {
                println!("{} \t| {:.2} \t\t| {:.2} |", f.day, f.interest, f.balance);
            }
        } else {
            println!("Account not found. Please register first.");
        }
    }
}


fn convert_amount(bank: &Bank, src_code: &str, dst_code: &str, amount: f64) -> Option<f64> {
    let src_rate = bank.forex.get_rate(src_code).copied()?;
    let dst_rate = bank.forex.get_rate(dst_code).copied()?;
    Some(amount * src_rate / dst_rate)
}

fn currency_menu_lists(bank: &Bank) -> (Vec<String>, Vec<String>) {
    let mut codes = Vec::new();
    let mut names = Vec::new();
    for c in bank.forex.currencies_detailed() {
        codes.push(c.code.clone());
        names.push(format!("{} ({})", c.name, c.code));
    }
    (codes, names)
}

fn print_currency_menu(names: &[String]) {
    for (i, name) in names.iter().enumerate() {
        println!("[{}] {}", i + 1, name);
    }
}

fn read_string_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn read_usize_prompt(prompt: &str) -> usize {
    loop {
        let s = read_string_prompt(prompt);
        if let Ok(v) = s.parse::<usize>() {
            if v > 0 {
                return v;
            }
        }
        println!("Please enter a valid number > 0.");
    }
}

fn read_f64_prompt(prompt: &str) -> f64 {
    loop {
        let s = read_string_prompt(prompt);
        if let Ok(v) = s.parse::<f64>() {
            if v > 0.0 {
                return v;
            }
        }
        println!("Please enter a valid amount > 0.");
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    loop {
        let s = read_string_prompt(prompt);
        let s = s.to_lowercase();
        if s.is_empty() || s == "y" || s == "yes" {
            return true;
        } else if s == "n" || s == "no" {
            return false;
        } else {
            println!("Please enter Y or N.");
        }
    }
}
