use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::product::{ProductsParam, Products};
use crate::util::{Deleted, Expandable, List};
use crate::Client;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub product: Option<Expandable<Products>>,
    pub tiers: Option<Tiers>,
    pub tiers_mode: Option<TiersMode>,
    pub transform_usage: Option<String>,
    pub trial_period_days: Option<i64>,
    pub usage_type: Option<UsageType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AggregateUsage {
    Sum,
    LastDuringPeriod,
    LastEver,
    Max,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tiers {
    pub amount: i64,
    pub up_to: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BillingScheme {
    PerUnit,
    Tiered,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TiersMode {
    Graduated,
    Volume,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TransformUsage {
    pub divide_by: f64,
    pub round: Round,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Round {
    UP,
    DOWN,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UsageType {
    Metered,
    Licensed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    Manual,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Default, Debug, Serialize, PartialEq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Plans {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Plans, vec![], param)
    }

    pub fn retrieve(client: &Client, plan: &str) -> crate::Result<Self> {
        client.get(UrlPath::Plans, vec![plan], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        plan: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Plans, vec![plan], param)
    }

    pub fn delete(client: &Client, plan: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Plans, vec![plan], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Plans, vec![], param)
    }
}
