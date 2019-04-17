use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::resources::core::product::ProductsParam;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::Deleted;
use crate::resources::common::path::StripePath;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plans {
    pub id: String,
    pub object: Object,
    pub active: bool,
    pub aggregate_usage: Option<AggregateUsage>,
    pub amount: i64,
    pub billing_scheme: Option<BillingScheme>,
    pub created: i64,
    pub currency: Currency,
    pub interval: Interval,
    pub interval_count: i64,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub nickname: Option<String>,
    pub product: Option<String>,
    pub tiers: Option<Tiers>,
    pub tiers_mode: Option<TiersMode>,
    pub transform_usage: Option<String>,
    pub trial_period_days: Option<i64>,
    pub usage_type: Option<UsageType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum AggregateUsage {
    Sum,
    LastDuringPeriod,
    LastEver,
    Max
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tiers {
    pub amount: i64,
    pub up_to: Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum BillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum TiersMode {
    Graduated,
    Volume
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TransformUsage {
    pub divide_by: f64,
    pub round: Round,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Round {
    UP,
    DOWN
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum UsageType {
    Metered,
    Licensed
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Interval {
    Manual,
    Day,
    Week,
    Month,
    Year
}

#[derive(Default, Debug, Serialize)]
pub struct PlansParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ProductsParam<'a>>,
}

impl StripeService for Plans {}
impl<'a> StripeService for PlansParam<'a> {}

impl Plans {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Plans, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, plan: &str) -> crate::Result<Self> {
        client.get(UrlPath::Plans, &StripePath::default().param(plan), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, plan: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Plans, &StripePath::default().param(plan), param)
    }

    pub fn delete(client: &Client, plan: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Plans, &StripePath::default().param(plan), Self::object())
    }
}