//use serde;
//use crate::stripe::{StripePath, UrlPath, Currency, Object};
//use crate::util::{List, Deleted, RangeQuery};
//use crate::{Client, Result};
//use std::collections::HashMap;
//use crate::StripeService;

use crate::{
    resources::common::{
        object::Object,
        currency::Currency,
        path::{UrlPath, StripePath}
    },
    util::{RangeQuery, Deleted, List},
    StripeService, Client
};
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
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
    pub valid: bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Duration {
    Forever,
    Once,
    Repeating
}

#[derive(Default, Serialize, Debug)]
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
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
}

#[derive(Default, Serialize, Debug)]
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

impl StripeService for Coupon {}
impl<'a> StripeService for CouponParam<'a> {}
impl<'a> StripeService for CouponListParam<'a> {}

impl Coupon {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Coupons, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, coupon: &str) -> crate::Result<Self> {
        client.get(UrlPath::Coupons, &StripePath::default().param(coupon), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, coupon: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Coupons, &StripePath::default().param(coupon), param)
    }

    pub fn delete(client: &Client, coupon: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Coupons, &StripePath::default().param(coupon), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Coupons, &StripePath::default(), param)
    }

}
