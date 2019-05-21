use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::util::{RangeQuery, Deleted, List};
use crate::Client;
use crate::resources::common::path::UrlPath;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Coupon {
    pub id: String,
    pub object: Object,
    pub amount_off: Option<i64>,
    pub created: i64,
    pub currency: Option<Currency>,
    pub duration: Duration,
    pub duration_in_months: Option<i32>,
    pub livemode: bool,
    pub max_redemptions: Option<i64>,
    pub metadata: HashMap<String, String>,
    pub name: Option<String>,
    pub percent_off: Option<f32>,
    pub redeem_by: Option<i64>,
    pub times_redeemed: i32,
    pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Duration {
    Forever,
    Once,
    Repeating,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct CouponParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct CouponListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl Coupon {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Coupons, vec![], param)
    }

    pub fn retrieve(client: &Client, coupon: &str) -> crate::Result<Self> {
        client.get(UrlPath::Coupons, vec![coupon], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        coupon: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Coupons, vec![coupon], param)
    }

    pub fn delete(client: &Client, coupon: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Coupons, vec![coupon], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Coupons, vec![], param)
    }
}

