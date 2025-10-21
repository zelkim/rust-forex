# Rust Forex & Banking Console App

A small Rust console application that lets you:
- Register and update foreign exchange (FX) rates
- Convert between currencies relative to a chosen base currency
- Manage a simple bank account (deposit/withdraw)
- Compute daily interest and show a day-by-day forecast

This project over-engineers the required features on purpose to practice clean API layering, documentation, and builder-style ergonomics in Rust.


## Architecture overview

Top-level modules are split between API types (logic) and the console view (UI):

- `src/api/`
  - `forex.rs` — In-memory FX registry/calculator
    - `Currency { code, name, rate }`
    - `Forex` with a currency catalog and a base currency
    - Builder-style methods to register currencies and set the base currency
    - Update-only `set_rate` to change an existing currency’s rate
  - `account.rs` — Account model and interest forecasting
    - `TransactionType` (Deposit | Withdraw)
    - `Transaction { value }` where withdraws are stored as negative values
    - `Account` holds name, transactions, and annual interest rate
    - Interest forecast using integer “day index” (no chrono)
  - `bank.rs` — Orchestrator
    - Holds a `Forex` instance, `annual_interest`, `base_currency`, and `accounts`
    - Builder methods to configure and finalize construction
    - `create_account`, `find_account`, `find_account_mut`
- `src/view/`
  - `console.rs` — Interactive console menu wiring the API together
  - `console_util.rs` — Input helpers, menu rendering, simple conversion helper used by the UI
- `src/main.rs` — Program entrypoint; wires up an initial Forex and Bank, then runs the console UI

Guiding principles:
- API is kept UI-agnostic. The console view talks only to the API.
- Builder-style methods consume and return `Self` for ergonomic chaining.
- FX “rate” means: price of 1 unit of that currency expressed in the base currency.


## How things work

### Forex
- The `Forex` catalog stores each `Currency` by code (e.g., "USD").
- Base currency (e.g., "PHP") is set once via `set_base_rate`. All `rate` values are defined relative to this base.
- `create_currency(code, name, rate)` registers currencies. Use it for all supported currencies.
- `set_rate(code, rate)` updates the rate of an existing currency only. It will NOT insert new currencies.
- `get_rate(code)` returns an `Option<&f64>` with the current rate.
- `currencies_detailed()` returns a sorted list of `Currency` for menus and diagnostics.

Conversion formula (src → dst):
- Given `rate_src` and `rate_dst` as amounts in base currency per 1 unit of src/dst:
  - `base_amount = amount_src * rate_src`
  - `amount_dst = base_amount / rate_dst`

### Bank
- Holds one `Forex`, a `base_currency` (a `Currency` struct), a default `annual_interest`, and a list of `Account`.
- `create_account(name)` creates a new account with the bank’s configured `annual_interest`.
- `find_account(_name)` and `find_account_mut(_name)` return references for reading/mutating.

### Account
- `create_transaction(Deposit|Withdraw, amount)` records positive amounts; withdraws are internally negative.
- `get_balance()` sums all transactions.
- `get_interest_forecast(days)` returns a `Vec<InterestForecast>` for Day 1..=days.
  - Daily Interest = End-of-Day Balance × (Annual Interest Rate / 365)
  - The forecast iterates by day over the current balance and interest rate to simulate compounding.

### Console UI
- Menus for: Register Account, Deposit, Withdraw, Currency Exchange, Record Exchange Rates, Show Interest.
- Input helpers validate numeric values must be greater than zero.
- Yes/No prompts accept Enter as Yes.
- Currency menus are generated from `Forex::currencies_detailed()` so they reflect the actual registry.


## Quick start

Prerequisites:
- Latest stable Rust toolchain from https://rustup.rs/

Build and run:
```sh
cargo build
cargo run
```

You’ll see a menu-driven console. Use the options to register accounts, record FX rates, and perform conversions.


## Create your own Bank with Forex conversions (minimal example)

Inside this repository, you can set up and use the API types directly, similar to `src/main.rs`.

```rust
use crate::api::{bank::Bank, forex::Forex};

fn make_bank() -> Bank {
    let forex = Forex::new()
        .create_currency("PHP", "Philippine Peso", 1.0)
        .create_currency("USD", "US Dollar", 58.1130)
        .create_currency("JPY", "Japanese Yen", 0.3865)
        .set_base_rate("PHP");

    Bank::new()
        .set_forex(forex)
        .set_annual_interest(0.05)
        .set_base_currency("PHP")
        .build()
}

fn convert_example(bank: &Bank, amount: f64, src: &str, dst: &str) -> Option<f64> {
    let rate_src = bank.forex.get_rate(src).copied()?; // price of 1 src in base (PHP)
    let rate_dst = bank.forex.get_rate(dst).copied()?; // price of 1 dst in base (PHP)
    let base_amount = amount * rate_src;               // convert src -> base
    Some(base_amount / rate_dst)                       // convert base -> dst
}
```

For account operations and interest forecast:

```rust
use crate::api::{account::TransactionType, bank::Bank};

fn account_flow(bank: &mut Bank) {
    let acct = bank.create_account("Alice");
    acct.create_transaction(TransactionType::Deposit, 1_000.0);
    acct.create_transaction(TransactionType::Withdraw, 250.0);

    println!("Balance: {:.2}", acct.get_balance());
    let forecast = acct.get_interest_forecast(7); // 7 days
    for day in forecast {
        println!("Day {}: interest {:.2}, balance {:.2}", day.day, day.interest, day.balance);
    }
}
```

Updating an existing exchange rate (no insert):

```rust
// Will only update if "USD" was previously created via create_currency
bank.forex.set_rate("USD", 58.42);
```


## Developer guide

### Project layout
- Keep domain logic in `src/api/*`. Avoid UI or I/O here.
- Keep console/UI logic in `src/view/*` and rely on the API only.
- `src/main.rs` wires app configuration and starts the console loop.

### Builder semantics
- Several API methods use a builder/fluent pattern by consuming and returning `Self`.
- This allows: `Forex::new().create_currency(...).set_base_rate("PHP")`.
- Because ownership moves each call, keep an eye on where you need `mut` vs. where you return `Self`.

### FX update-only policy
- `set_rate` updates existing currencies but does not insert new ones.
- Ensure all currencies appear via `create_currency` first (e.g., at startup).

### Extending the console
- Add a new menu item in `src/view/console.rs` and consider extracting helpers to `console_util.rs` if reusable.
- Keep prompts robust: reuse `read_*_prompt` helpers and input validations (> 0 for numerics).

### Testing suggestions
- Add tests under `tests/` or unit tests alongside modules.
- Recommended small tests:
  - Forex conversion round-trips (src -> base -> src)
  - `set_rate` update behavior (no insert)
  - Account deposit/withdraw and interest forecast for a few days

Example test sketch:

```rust
#[test]
fn updates_rate_without_insert() {
    let mut fx = Forex::new().create_currency("USD", "US Dollar", 58.0).set_base_rate("PHP");
    assert_eq!(fx.get_rate("USD"), Some(&58.0));
    fx.set_rate("USD", 59.0);
    assert_eq!(fx.get_rate("USD"), Some(&59.0));
    fx.set_rate("EUR", 67.0); // not previously created
    assert_eq!(fx.get_rate("EUR"), None);
}
```

### Troubleshooting
- “Rate not updated” after calling `set_rate`: make sure the currency was registered via `create_currency`.
- Float comparisons: use a small epsilon if checking equality (`(a - b).abs() < 1e-9`).
- Borrow checker issues: when needing to read some data before a mutable borrow, clone what you need (e.g., currency code strings) to avoid conflicts.

### Style and tools
- Use `rustfmt` to keep style consistent.
- Consider `clippy` for lints.


## Spec compliance quick map
See `spec.md` for the full table. This app implements:
- REQ-0001: Record/register exchange rates for USD, JPY, GBP, EUR, CNY (and base PHP)
- REQ-0002: Set PHP as base currency
- REQ-0003: Convert from one currency to another (via base)
- REQ-0004: Deposit money to a user account
- REQ-0005: Withdraw money from a user account
- REQ-0006: Compute daily interest (5% per annum by default)
- REQ-0007: Show daily interest forecast for N days
- REQ-0008: Transact in foreign currencies (via FX and conversion)
- REQ-0009: Register/input user details (account name)
- REQ-0010: Built in Rust

---

If you want to use these APIs from a separate project as a library, consider adding a `src/lib.rs` that re-exports the `api` modules; then depend on this crate from your application. For now, this repository is structured as a binary application.