use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::resources::core::disputes::{DisputeReason, DisputeStatus};
use crate::Client;
use crate::resources::common::path::UrlPath;
use crate::util::{List, RangeQuery, Expandable};
use crate::resources::issuing::transactions::Transactions;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssuingDispute {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub created: i64,
    pub currency: Currency,
    pub disputed_transaction: Option<Expandable<Transactions>>,
    pub evidence: DisputeEvidence,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub reason: DisputeReason,
    pub status: DisputeStatus
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DisputeEvidence {
    pub fraudulent: Option<Evidence>,
    pub other: Option<Evidence>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Evidence {
    pub dispute_explanation: Option<String>,
    pub uncategorized_file: Option<String>,
}

#[derive(Default, Debug, Serialize)]
pub struct IssuingDisputeParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputed_transaction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<DisputeReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<DisputeEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct IssuingDisputeListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disputed_transaction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}


impl IssuingDispute {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingDispute, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::IssuingDispute, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingDispute, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::IssuingDispute, vec![], param)
    }
}