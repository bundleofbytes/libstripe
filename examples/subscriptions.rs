use libstripe::resources::billing::subscriptions::{ItemParam, Subscription, SubscriptionParam};
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
    customer_param.source = Some(PaymentSourceParam::Token("tok_visa"));

    let customer = Customer::create(&client, customer_param)?;

    let mut subscription_param = SubscriptionParam::default();

    let mut items = ItemParam::default();
    items.plan = "golden-plan";

    subscription_param.items = Some(vec![items]);
    subscription_param.customer = Some(&customer.id);

    let subscription = Subscription::create(&client, subscription_param)?;

    println!("{:?}", subscription);

    Ok(())
}
