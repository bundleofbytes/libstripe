use crate::resources::common::object::Object;
use crate::resources::billing::coupons::Coupon;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;
use crate::util::Deleted;

#[derive(Deserialize, Debug)]
pub struct Discount {
    pub object: Object,
    pub coupon: Coupon,
    pub customer: String,
    pub end: i64,
    pub start: i64,
    pub subscription: Option<String>,
}

impl StripeService for Discount {}

impl Discount {

    pub fn delete_customer_discount(client: &Client, cust_id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Customers, &StripePath::default().param(cust_id).param("discount"), Self::object())
    }

    pub fn delete_subscription_discount(client: &Client, sub_id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Subscriptions, &StripePath::default().param(sub_id).param("discount"), Self::object())
    }

}