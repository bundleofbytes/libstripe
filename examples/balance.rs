use libstripe::resources::core::balance::Balance;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let balance = Balance::retrieve(&client)?;

    println!("{:?}", balance);
    Ok(())
}
