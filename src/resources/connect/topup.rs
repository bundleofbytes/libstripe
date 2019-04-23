use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::paymentmethods::source::Source;
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::issuing::transactions::Transactions;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Topup {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<Expandable<Transactions>>,
    pub created: i64,
    pub currency: Currency,
    pub description: String,
    pub exepcted_availability_date: Option<i64>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub source: Source,
    pub statement_descriptor: Option<String>,
    pub status: TopupStatus,
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TopupStatus {
    Succeeded,
    Pending,
    Reversed,
    Failed,
    Canceled,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct TopupParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct TopupListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TopupStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Topup {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Topups, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Topups, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Topups, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Topups, vec![], param)
    }

    pub fn cancel(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Topups, vec![id, "cancel"], serde_json::Map::new())
    }
}
