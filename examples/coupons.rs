
use std::env;
use libstripe::Client;
use libstripe::resources::billing::coupons::{Duration, Coupon, CouponParam};

fn main() -> libstripe::Result<()> {
    let secret_key = env::var("STRIPE_KEY").expect("Missing 'STRIPE_KEY'.");
    let client = Client::new(&secret_key);
    
    let mut coupon_param = CouponParam::default();
    coupon_param.percent_off = Some(25);
    coupon_param.duration = Some(Duration::Repeating);
    coupon_param.duration_in_months = Some(3);
    coupon_param.id = Some("250FF");

    let coupon = Coupon::create(&client, coupon_param)?;

    println!("{:?}", coupon);

    Ok(())
}