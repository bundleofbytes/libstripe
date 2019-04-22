use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::balance::BalanceTransaction;
use crate::util::{List, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::refunds::Refund;
use crate::resources::connect::transfers::Transfer;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferReversal {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,
    pub created: i64,
    pub currency: Currency,
    pub destination_payment_refund: Option<Expandable<Refund>>,
    pub metadata: HashMap<String, String>,
    pub source_refund: Option<Expandable<Refund>>,
    pub transfer: Expandable<Transfer>,
}

#[derive(Default, Serialize, Debug)]
pub struct TransferReveralParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct TransferReversalListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl TransferReversal {
    pub fn create<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transfers, vec![id, "reversals"], param)
    }

    pub fn retrieve(client: &Client, id: &str, reversal_id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::Transfers,
            vec![id, "reversals", reversal_id],
            serde_json::Map::new(),
        )
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        id: &str,
        reversal_id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(
            UrlPath::Transfers,
            vec![id, "reversals", reversal_id],
            param,
        )
    }

    pub fn list<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(UrlPath::Transfers, vec![id, "reversals"], param)
    }
}
