use libstripe::resources::billing::invoiceitems::{InvoiceItems, InvoiceItemsParam};
use libstripe::resources::common::currency::Currency;
use libstripe::resources::core::customer::{Customer, CustomerParam};
use libstripe::resources::paymentmethods::source::PaymentSourceParam;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut customer_param = CustomerParam::default();
    customer_param.email = Some("example@example.com");
    customer_param.description = Some("Example account");
    customer_param.source = Some(PaymentSourceParam::Token("tok_amex"));

    let customer = Customer::create(&client, customer_param)?;

    let mut invoiceitems_param = InvoiceItemsParam::default();
    invoiceitems_param.amount = Some(2500);
    invoiceitems_param.currency = Some(Currency::default());
    invoiceitems_param.customer = Some(&customer.id);
    invoiceitems_param.description = Some("One-time setup fee");

    let invoiceitems = InvoiceItems::create(&client, invoiceitems_param)?;

    println!("{:?}", invoiceitems);

    Ok(())
}
