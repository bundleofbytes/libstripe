use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::balance::BalanceTransaction;
use crate::util::{List, Expandable};
use crate::{Client};
use serde::Serialize;
use std::collections::HashMap;
use crate::resources::connect::applicationfees::ApplicationFees;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationFeeRefunds {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,
    pub created: i64,
    pub currency: Currency,
    pub fee: Expandable<ApplicationFees>,
    pub metadata: HashMap<String, String>,
}

#[derive(Default, Debug, Serialize)]
pub struct ApplicationFeeRetundsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
}

#[derive(Default, Debug, Serialize)]
pub struct ApplicationFeeRetundsListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl ApplicationFeeRefunds {
    pub fn create<B: Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::ApplicationFees, vec![id, "refunds"], param)
    }

    pub fn retrieve(client: &Client, id: &str, fee: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::ApplicationFees,
            vec![id, "refunds", fee],
            serde_json::Map::new(),
        )
    }

    pub fn update<B: Serialize>(
        client: &Client,
        id: &str,
        refund: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::ApplicationFees, vec![id, "refunds", refund], param)
    }

    pub fn list<B: Serialize>(client: &Client, id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::ApplicationFees, vec![id], param)
    }
}
