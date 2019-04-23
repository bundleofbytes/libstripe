use crate::resources::billing::coupons::Coupon;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::Deleted;
use crate::{Client};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Discount {
    pub object: Object,
    pub coupon: Coupon,
    pub customer: String,
    pub end: i64,
    pub start: i64,
    pub subscription: Option<String>,
}

impl Discount {
    pub fn delete_customer_discount(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(
            UrlPath::Customers,
            vec![id, "discount"],
            serde_json::Map::new(),
        )
    }

    pub fn delete_subscription_discount(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(
            UrlPath::Subscriptions,
            vec![id, "discount"],
            serde_json::Map::new(),
        )
    }
}
