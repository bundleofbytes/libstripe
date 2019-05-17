use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::refunds::Refund;
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use crate::resources::connect::account::Account;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::core::charges::Charge;
use crate::resources::connect::transfers::Transfer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ApplicationFees {
    pub id: String,
    pub object: Object,
    pub account: Expandable<Account>,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: String,
    pub balance_transaction: Expandable<BalanceTransaction>,
    pub charge: Expandable<Charge>,
    pub created: i64,
    pub currency: Currency,
    pub livemode: bool,
    pub originating_transaction: Option<Expandable<OriginatingTransaction>>,
    pub refunded: bool,
    pub refunds: List<Refund>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum OriginatingTransaction {
    Charge(Charge),
    Transfer(Transfer)
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct ApplicationFeesParam<'a>{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ApplicationFeesListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl ApplicationFees {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::ApplicationFees, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::ApplicationFees, vec![], param)
    }
}
