use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::resources::paymentmethods::source::Source;
use crate::util::{RangeQuery, List};
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct Topup {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<String>,
    pub created: i64,
    pub currency: Currency,
    pub description: String,
    pub exepcted_availability_date: i64,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub source: Source,
    pub statement_descriptor: Option<String>,
    pub status: TopupStatus,
    pub transfer_group: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum TopupStatus {
    Succeeded,
    Pending,
    Reversed,
    Failed,
    Canceled
}

#[derive(Default, Serialize, Debug)]
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
    pub transfer_group: Option<&'a str>
}

#[derive(Default, Serialize, Debug)]
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
}


impl StripeService for Topup {}
impl<'a> StripeService for TopupParam<'a> {}
impl<'a> StripeService for TopupListParams<'a> {}

impl Topup {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Topups, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Topups, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Topups, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Topups, &StripePath::default(), param)
    }

    pub fn cancel(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Topups, &StripePath::default().param(id).param("cancel"), Self::object())
    }

}