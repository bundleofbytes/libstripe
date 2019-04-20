use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::{Client};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payout {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub arrival_date: i64,
    pub automatic: bool,
    pub balance_transaction: String,
    pub created: i64,
    pub currency: Currency,
    pub description: String,
    pub destination: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub failure_code: Option<PayoutFailureCode>,
    pub failure_message: Option<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub method: PayoutMethod,
    pub source_type: PayoutSourceType,
    pub statement_descriptor: Option<String>,
    pub status: PayoutStatus,
    #[serde(rename = "type")]
    pub payout_type: PayoutType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PayoutStatus {
    Paid,
    Pending,
    InTransit,
    Canceled,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PayoutSourceType {
    BankAccount,
    Card,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PayoutFailureCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    CouldNotProcess,
    DebitNotAuthorized,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    UnsupportedCard,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PayoutMethod {
    STANDARD,
    INSTANT,
}

#[derive(Default, Serialize, Debug)]
pub struct PayoutParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PayoutMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PayoutType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Payout {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Payouts, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Payouts, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Payouts, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Payouts, vec![], param)
    }

    pub fn cancel(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Payouts, vec![id, "cancel"], serde_json::Map::new())
    }
}
