mod api { pub mod account; pub mod bank; pub mod forex; }
mod view { pub mod console; }
use api::forex::Forex;
use api::bank::Bank;
use view::console::ConsoleApp;

fn main() {
    // Initial exchange rate retrieved from bsp.gov.ph on 10/20/2025
    let forex = Forex::new()
        .create_currency("PHP", "Philippine Peso", 1.0)
        .create_currency("USD", "US Dollar", 58.1130)
        .create_currency("JPY", "Japanese Yen", 0.3865)
        .create_currency("GBP", "British Pound", 78.0632)
        .create_currency("EUR", "Euro", 67.7598)
        .create_currency("CNY", "Chinese Yuan", 8.1531)
        .set_base_rate("PHP");

    let bank = Bank::new()
        .set_forex(forex)
        .set_annual_interest(0.05)
        .set_base_currency("PHP")
        .build();

    let mut app = ConsoleApp::new(bank);
    app.run();
}