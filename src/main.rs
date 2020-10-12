#![allow(non_snake_case)]

use std::env;
use serde::Deserialize;

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
    let resp = reqwest::get("https://api.exchangeratesapi.io/latest?base=AUD&symbols=USD,GBP,EUR")
        .await?
        .json::<ResponseData>()
        .await?;

    println!("{:#?}", resp);

    let aud_to_usd_rate = resp.rates.USD;

    let args: Vec<String> = env::args().collect();

    let input_amount = &args[1];
    let input_currency = &args[2];
    let output_currency = &args[3];
    let mut output_amount = args[1].parse::<f32>().unwrap();
    output_amount = output_amount * aud_to_usd_rate;

    println!("{} {} is {} {}", input_currency, input_amount, output_currency, output_amount);

    return Ok(());
}
