
use std::env;
use libstripe::Client;
use libstripe::resources::common::currency::Currency;
use libstripe::resources::core::payout::{Payout, PayoutParam};

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut payout_param = PayoutParam::default();
    payout_param.amount = Some(400);
    payout_param.currency = Some(Currency::USD);

    let payout = Payout::create(&client, payout_param)?;

    println!("{:?}", payout);

    Ok(())
}