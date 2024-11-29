use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ExchangeRate {
    rates: std::collections::HashMap<String, f64>,
}

async fn fetch_exchange_rate(client: &Client, base: &str) -> Result<ExchangeRate, reqwest::Error> {
    let url = format!("https://api.exchangerate-api.com/v4/latest/{}", base);
    let response = client.get(&url).send().await?.json::<ExchangeRate>().await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <amount> <base_currency> <target_currency>", args[0]);
        return;
    }

    let amount: f64 = args[1].parse().expect("Amount must be a number");
    let base_currency = &args[2];
    let target_currency = &args[3];

    let client = Client::new();

    match fetch_exchange_rate(&client, base_currency).await {
        Ok(exchange_rate) => {
            if let Some(rate) = exchange_rate.rates.get(target_currency) {
                let converted_amount = amount * rate;
                println!("{} {} = {} {}", amount, base_currency, converted_amount, target_currency);
            } else {
                eprintln!("Target currency {} not found in the exchange rates.", target_currency);
            }
        }
        Err(e) => {
            eprintln!("Error fetching exchange rate: {}", e);
        }
    }
}
