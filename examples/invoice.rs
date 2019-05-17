use libstripe::resources::billing::invoices::{Invoice, InvoiceParam};
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

    let mut invoice_param = InvoiceParam::default();
    invoice_param.customer = Some(&customer.id);

    let invoice = Invoice::create(&client, invoice_param)?;

    println!("{:?}", invoice);

    Ok(())
}
