use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::issuing::authorizations::MerchantData;
use crate::util::{List, RangeQuery};
use crate::{Client};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Transactions {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub authorization: String,
    pub balance_transaction: String,
    pub card: String,
    pub cardholder: String,
    pub created: i64,
    pub currency: Currency,
    pub dispute: String,
    pub livemode: bool,
    pub merchant_data: MerchantData,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Capture,
    Refund,
    CashWithdrawal,
    RefundReversal,
    Dispute,
    DisputeLoss,
}

#[derive(Serialize, Debug)]
pub struct TransactionsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Serialize, Debug)]
pub struct TransactionsListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}


impl Transactions {

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Transactions, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transactions, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Transactions, vec![], param)
    }

}
