use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use crate::resources::orders::order::OrderItem;
use crate::util::{RangeQuery, List};
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Debug, Deserialize)]
pub struct Returns {
    pub id: String,
    pub object: Object,
    pub amount: i32,
    pub created: i64,
    pub currency: Currency,
    pub items: Vec<OrderItem>,
    pub livemode: bool,
    pub order: String,
    pub refund: Option<String>
}

#[derive(Default, Serialize, Debug)]
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

impl StripeService for Returns {}
impl<'a> StripeService for ReturnListParams<'a> {}

impl Returns {

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::OrderReturns, &StripePath::default().param(id), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::OrderReturns, &StripePath::default(), param)
    }

}