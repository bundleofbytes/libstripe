use libstripe::resources::billing::invoices::{Invoice, InvoiceParam};
use libstripe::resources::core::customer::{Customer, CustomerParam};
use libstripe::resources::paymentmethods::source::PaymentSourceParam;
use libstripe::Client;
use std::env;
use libstripe::resources::billing::invoiceitems::{InvoiceItemsParam, InvoiceItems};
use libstripe::resources::common::currency::Currency;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut customer_param = CustomerParam::default();
    customer_param.email = Some("example@example.com");
    customer_param.description = Some("Example account");
    customer_param.source = Some(PaymentSourceParam::Token("tok_amex"));

    let customer = Customer::create(&client, customer_param)?;

    let mut item_param = InvoiceItemsParam::default();
    item_param.currency = Some(Currency::USD);
    item_param.customer = Some(&customer.id);
    item_param.amount = Some(100);
    item_param.description = Some("Example");

    let _ = InvoiceItems::create(&client, item_param)?;

    let mut invoice_param = InvoiceParam::default();
    invoice_param.customer = Some(&customer.id);

    let invoice = Invoice::create(&client, invoice_param)?;

    println!("{:?}", invoice);

    Ok(())
}
