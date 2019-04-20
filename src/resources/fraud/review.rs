use crate::resources::common::object::Object;
use crate::Client;
use crate::resources::common::path::UrlPath;
use crate::util::RangeQuery;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reviews {
    pub id: String,
    pub object: Object,
    pub billing_zip: Option<String>,
    pub charge: Option<String>,
    pub closed_reason: Reason,
    pub created: i64,
    pub ip_address: Option<String>,
    pub ip_address_locations: Option<String>,
    pub livemode: bool,
    pub opened_reason: Reason,
    pub session: String,
    pub payment_intent: Option<String>,
    pub open: bool,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub browser: String,
    pub device: String,
    pub platform: String,
    pub version: String

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Reason {
    Rule,
    Manual,
    Approved,
    Refunded,
    RefundedAsFraud,
    Disputed,
}

#[derive(Default, Serialize, Debug)]
pub struct ReviewsListParams<'a> {
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

impl Reviews {

    pub fn approve(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Reviews, vec![id, "approve"], serde_json::Map::new())
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Reviews, vec![id], serde_json::Map::new())
    }

}