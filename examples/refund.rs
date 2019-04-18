use libstripe::resources::common::currency::Currency;
use libstripe::resources::core::charges::{Charge, ChargeParams};
use libstripe::resources::core::refunds::{Refund, RefundParam};
use libstripe::resources::paymentmethods::source::PaymentSourceParam;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut charge_param = ChargeParams::default();
    charge_param.amount = Some(2000);
    charge_param.currency = Some(Currency::USD);
    charge_param.source = Some(PaymentSourceParam::Token("tok_visa"));

    let charge = Charge::create(&client, charge_param)?;

    let mut refund_param = RefundParam::default();
    refund_param.charge = Some(&charge.id);

    let refund = Refund::create(&client, refund_param)?;

    println!("{:?}", refund);

    Ok(())
}
