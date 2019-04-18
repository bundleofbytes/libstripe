use libstripe::resources::core::customer::{Customer, CustomerParam};
use libstripe::resources::paymentmethods::bank::{
    AccountHolderType, BankAccount, BankAccountParam,
};
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

    let mut bank_param = BankAccountParam::default();
    bank_param.account_number = Some("000123456789"); //successful account number
    bank_param.routing_number = Some("110000000");
    bank_param.country = Some("US");
    bank_param.account_holder_name = Some("John Doe");
    bank_param.account_holder_type = Some(AccountHolderType::Individual);

    let mut payment_param = PaymentParam::default();
    payment_param.source = Some(PaymentSourceParam::BankAccount(bank_param));

    let bank = BankAccount::create(&client, &customer.id, payment_param)?;

    println!("{:?}", bank);
    Ok(())
}
