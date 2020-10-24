#![allow(non_snake_case)]

use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
struct ResponseData {
    rates: HashMap<String, f32>,
    base: String,
    date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input_amount = &args[1];
    let input_currency = &args[2].to_uppercase();
    let output_currency = &args[3].to_uppercase();

    let url = format!(
        "https://api.exchangeratesapi.io/latest?base={}&symbols={}",
        input_currency, output_currency
    );
    // println!("{:#?}", url);
    // TODO: Error handling if not 200
    println!("Fetching conversion rates...");
    let resp = reqwest::get(&url).await?.json::<ResponseData>().await?;
    // println!("{:#?}", resp);

    // QUESTION: I still don't understand * and &
    // TODO: error handling
    let conversion_rate = *resp.rates.get(output_currency).unwrap();
    println!("Conversion rate is: {}", conversion_rate);

    let parsed_input = input_amount.parse::<f32>().unwrap();
    let output_amount = parsed_input * conversion_rate;

    println!(
        "{}{} is {}{}",
        input_currency, input_amount, output_currency, output_amount
    );

    return Ok(());
}
