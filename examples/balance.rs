

use std::env;
use libstripe::Client;
use libstripe::resources::core::balance::Balance;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let balance = Balance::retrieve(&client)?;

    println!("{:?}", balance);
    Ok(())
}