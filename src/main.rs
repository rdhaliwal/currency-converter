#![allow(non_snake_case)]

use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct RatesModel {
    EUR: f32,
    USD: f32,
    GBP: f32,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    rates: RatesModel,
    base: String,
    date: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let input_amount = &args[1];
    let input_currency = &args[2];
    let output_currency = &args[3];
    let url = format!("https://api.exchangeratesapi.io/latest?base=AUD&symbols=USD,EUR,GBP");
    // let url = format!("https://api.exchangeratesapi.io/latest?base=AUD&symbols=USD,EUR", input_currency, output_currency);

    let resp = reqwest::get(&url).await?.json::<ResponseData>().await?;

    println!("{:#?}", resp);

    let aud_to_usd_rate = resp.rates.USD;

    let mut output_amount = input_amount.parse::<f32>().unwrap();
    output_amount = output_amount * aud_to_usd_rate;

    println!(
        "{} {} is {} {}",
        input_currency, input_amount, output_currency, output_amount
    );

    return Ok(());
}
