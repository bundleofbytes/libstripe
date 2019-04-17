
use std::env;
use libstripe::Client;
use libstripe::resources::core::customer::{Customer, CustomerParam};

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut param = CustomerParam::default();
    param.email = Some("j.doe@example.com");
    param.description = Some("Example Account");

    let customer = Customer::create(&client.clone(), param).unwrap();
    println!("{:?}", customer);

    let deleted = Customer::delete(&client, &customer.id)?;
    println!("{:?}", deleted);

    Ok(())
}