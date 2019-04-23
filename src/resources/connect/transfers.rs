use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::connect::transfer_reversal::TransferReversal;
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::connect::account::Account;
use crate::resources::issuing::transactions::Transactions;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transfer {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub amount_reversed: i64,
    pub balance_transaction: Expandable<BalanceTransaction>,
    pub created: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub destination: Expandable<Account>,
    pub destination_payment: String, //Expandable but needs clarification
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub reversals: List<TransferReversal>,
    pub reversed: bool,
    pub source_transaction: Option<Expandable<Transactions>>,
    pub source_type: TransferSourceType,
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransferSourceType {
    Card,
    BankAccount,
    AlipayAccount,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct TransferParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct TransferListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Transfer {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transfers, vec![], param)
    }

    pub fn retrieve<B: serde::Serialize>(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Transfers, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transfers, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Transfers, vec![], param)
    }
}
