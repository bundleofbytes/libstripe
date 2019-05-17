use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::orders::order::{OrderItem, Order};
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use crate::resources::core::refunds::Refund;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Returns {
    pub id: String,
    pub object: Object,
    pub amount: i32,
    pub created: i64,
    pub currency: Currency,
    pub items: Vec<OrderItem>,
    pub livemode: bool,
    pub order: Expandable<Order>,
    pub refund: Option<Expandable<Refund>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ReturnListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<&'a str>,
}

impl Returns {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::OrderReturns, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::OrderReturns, vec![], param)
    }
}
