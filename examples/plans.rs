use libstripe::resources::billing::plans::{Interval, Plans, PlansParam};
use libstripe::resources::common::currency::Currency;
use libstripe::resources::core::product::ProductsParam;
use libstripe::Client;
use std::env;

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);

    let mut plans_param = PlansParam::default();
    plans_param.id = Some("golden-plan");
    plans_param.amount = Some(5000);
    plans_param.currency = Some(Currency::default());
    plans_param.interval = Some(Interval::Month);
    plans_param.nickname = Some("Golden Plan");

    let mut product_param = ProductsParam::default();
    product_param.name = Some("Golden Plan");
    plans_param.product = Some(product_param);

    let plan = Plans::create(&client, plans_param)?;

    println!("{:?}", plan);

    //    let deleted = Plans::delete(&client, &plan.id)?;

    //    println!("{:?}", deleted);

    Ok(())
}
