use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::balance::BalanceTransaction;
use crate::util::List;
use crate::{Client};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ApplicationFeeRefunds {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<BalanceTransaction>,
    pub created: i64,
    pub currency: Currency,
    pub fee: String,
    pub metadata: HashMap<String, String>,
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
