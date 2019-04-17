
use std::env;
use libstripe::Client;
use libstripe::resources::common::currency::Currency;
use libstripe::resources::core::charges::{Charge, ChargeParams};
use libstripe::resources::paymentmethods::source::PaymentSourceParam;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);
    
    let mut charge_param = ChargeParams::default();
    charge_param.amount = Some(2000);
    charge_param.currency = Some(Currency::USD);
    charge_param.source = Some(PaymentSourceParam::Token("tok_visa"));

    let charge = Charge::create(&client, charge_param)?;

    println!("{:?}", charge);

    Ok(())

}