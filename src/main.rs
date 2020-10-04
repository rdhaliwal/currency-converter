use std::env;

fn main() {
    let AUD_TO_USD = 0.729746;

    let args: Vec<String> = env::args().collect();

    let input_amount = &args[1];
    let input_currency = &args[2];
    let output_currency = &args[3];
    let mut output_amount = args[1].parse::<f32>().unwrap();
    output_amount = output_amount / AUD_TO_USD;

    println!("Converting {} {}", input_currency, input_amount);
    println!("Which is also {} {}", output_currency, output_amount);
}
