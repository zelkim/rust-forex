use std::io::{self, Write};

use crate::api::bank::Bank;

pub fn convert_amount(bank: &Bank, src_code: &str, dst_code: &str, amount: f64) -> Option<f64> {
    let src_rate = bank.forex.get_rate(src_code).copied()?;
    let dst_rate = bank.forex.get_rate(dst_code).copied()?;
    Some(amount * src_rate / dst_rate)
}

pub fn currency_menu_lists(bank: &Bank) -> (Vec<String>, Vec<String>) {
    let mut codes = Vec::new();
    let mut names = Vec::new();
    for c in bank.forex.currencies_detailed() {
        codes.push(c.code.clone());
        names.push(format!("{} ({})", c.name, c.code));
    }
    (codes, names)
}

pub fn print_currency_menu(names: &[String]) {
    for (i, name) in names.iter().enumerate() {
        println!("[{}] {}", i + 1, name);
    }
}

pub fn read_string_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

pub fn read_usize_prompt(prompt: &str) -> usize {
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

pub fn read_f64_prompt(prompt: &str) -> f64 {
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

pub fn ask_yes_no(prompt: &str) -> bool {
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
