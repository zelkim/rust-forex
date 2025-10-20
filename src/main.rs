mod forex; 
use forex::Forex;

fn main() {
    println!("Hello, world!");

    // Initial exchange rate retrieved from bsp.gov.ph on 10/20/2025
    let mut forex = Forex::new()
        .create_rate("PHP", 1.0)
        .create_rate("USD", 58.1130)
        .create_rate("JPY", 0.3865)
        .create_rate("GBP", 78.0632)
        .create_rate("EUR", 67.7598)
        .create_rate("CNY", 8.1531)
        .set_base_rate("PHP");
    
    println!("Available currencies: {:?}", forex.currencies());
}