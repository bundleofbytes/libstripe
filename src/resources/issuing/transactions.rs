use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::issuing::authorizations::{MerchantData, Authorizations};
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::paymentmethods::cards::Card;
use crate::resources::issuing::cardholders::CardHolders;
use crate::resources::core::disputes::Dispute;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transactions {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub authorization: Expandable<Authorizations>,
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,
    pub card: Expandable<Card>,
    pub cardholder: Option<Expandable<CardHolders>>,
    pub created: i64,
    pub currency: Currency,
    pub dispute: Option<Expandable<Dispute>>,
    pub livemode: bool,
    pub merchant_data: MerchantData,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Capture,
    Refund,
    CashWithdrawal,
    RefundReversal,
    Dispute,
    DisputeLoss,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct TransactionsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
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
