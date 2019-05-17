use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::{List, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::core::charges::Charge;
use crate::resources::connect::transfer_reversal::TransferReversal;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Refund {
    pub id: String,
    pub object: Object,
    pub amount: i32,
    pub balance_transaction: Expandable<BalanceTransaction>,
    pub charge: Expandable<Charge>,
    pub created: i64,
    pub currency: Currency,
    pub metadata: HashMap<String, String>,
    pub reason: Option<RefundReason>,
    pub failure_balance_transaction: Option<Expandable<BalanceTransaction>>,
    pub failure_reason: Option<FailureReason>,
    pub source_transfer_reversal: Option<Expandable<TransferReversal>>,
    pub transfer_reversal: Option<Expandable<TransferReversal>>,
    pub receipt_number: Option<String>,
    pub status: RefundStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FailureReason {
    LostOrStolenCard,
    ExpiredOrCanceledCard,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RefundStatus {
    Succeeded,
    Pending,
    Failed,
    Cancelled,
}

#[derive(Default, Serialize, Debug, PartialEq)]
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
    pub reverse_transfer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Refund {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Refunds, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Refunds, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Refunds, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Refunds, vec![], param)
    }
}
