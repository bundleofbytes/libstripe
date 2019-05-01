use crate::resources::common::object::Object;
use std::collections::HashMap;
use crate::Client;
use crate::resources::common::path::UrlPath;
use crate::util::{List, RangeQuery};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TaxRate {
    pub id: String,
    pub object: Object,
    pub active: bool,
    pub created: i64,
    pub description: String,
    pub display_name: String,
    pub inclusive: bool,
    pub jurisdiction: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub percentage: f64
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct TaxRateParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct TaxRateListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl TaxRate {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TaxRates, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::TaxRates, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TaxRates, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::TaxRates, vec![], param)
    }
}