use libstripe::resources::connect::country::CountrySpecs;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let countries = CountrySpecs::retrieve(&client, "us")?;

    println!("{:?}", countries);
    Ok(())
}
