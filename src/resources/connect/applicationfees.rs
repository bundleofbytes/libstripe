use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::refunds::Refund;
use crate::util::List;
use crate::{Client};

#[derive(Deserialize, Debug)]
pub struct ApplicationFees {
    pub id: String,
    pub object: Object,
    pub account: String,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: String,
    pub balance_transaction: String,
    pub charge: String,
    pub created: i64,
    pub currency: Currency,
    pub livemode: bool,
    pub originating_transaction: Option<String>,
    pub refunded: bool,
    pub refunds: List<Refund>,
}

impl ApplicationFees {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::ApplicationFees, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::ApplicationFees, vec![], param)
    }
}
