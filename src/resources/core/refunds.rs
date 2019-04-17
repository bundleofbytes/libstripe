use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::resources::common::path::StripePath;

#[derive(Debug, Deserialize)]
pub struct Refund {
    pub id: String,
    pub object: Object,
    pub amount: i32,
    pub balance_transaction: String,
    pub charge: String,
    pub created: i64,
    pub currency: Currency,
    pub metadata: HashMap<String, String>,
    pub reason: Option<RefundReason>,
    pub failure_balance_transaction: Option<String>,
    pub failure_reason: Option<FailureReason>,
    pub source_transfer_reversal: Option<String>,
    pub transfer_reversal: Option<String>,
    pub receipt_number: Option<String>,
    pub status: RefundStatus,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum FailureReason {
    LostOrStolenCard,
    ExpiredOrCanceledCard,
    Unknown
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum RefundStatus {
    Succeeded,
    Pending,
    Failed,
    Cancelled
}

#[derive(Default, Serialize, Debug)]
pub struct RefundParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>
}

impl StripeService for Refund {}
impl<'a> StripeService for RefundParam<'a> {}


impl Refund {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Refunds, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Refunds, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Refunds, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Refunds, &StripePath::default(), param)
    }

}