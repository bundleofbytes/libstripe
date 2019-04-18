use libstripe::resources::common::currency::Currency;
use libstripe::resources::connect::transfers::{Transfer, TransferParam};
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let destination = env::var("DESTINATION_KEY").expect("Missing 'DESTINATION_KEY'.");

    let client = Client::new(&secret_key);

    let mut transfer_param = TransferParam::default();

    transfer_param.amount = Some(400);
    transfer_param.currency = Some(Currency::USD);
    transfer_param.destination = Some(&destination);

    let transfer = Transfer::create(&client, transfer_param)?;

    println!("{:?}", transfer);

    Ok(())
}
