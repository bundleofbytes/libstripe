use libstripe::resources::core::customer::{Customer, CustomerParam};
use libstripe::resources::paymentmethods::cards::Card;
use libstripe::resources::paymentmethods::source::{PaymentParam, PaymentSourceParam};
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut customer_param = CustomerParam::default();
    customer_param.email = Some("example@example.com");
    customer_param.description = Some("Example account");
    let customer = Customer::create(&client, customer_param)?;

    let mut payment_param = PaymentParam::default();
    payment_param.source = Some(PaymentSourceParam::Token("tok_visa"));

    let card = Card::create(&client, &customer.id, payment_param)?;

    println!("{:?}", card);

    Ok(())
}
