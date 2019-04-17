use crate::resources::common::object::Object;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::{StripeService, Client};
use serde::Serialize;
use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct ApplicationFeeRefunds {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<BalanceTransaction>,
    pub created: i64,
    pub currency: Currency,
    pub fee: String,
    pub metadata: HashMap<String, String>
}

impl StripeService for ApplicationFeeRefunds {}

impl ApplicationFeeRefunds {

    pub fn create<B: Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::ApplicationFees, &StripePath::default().param(id).param("refunds"), param)
    }

    pub fn retrieve(client: &Client, id: &str, fee: &str) -> crate::Result<Self> {
        client.get(UrlPath::ApplicationFees, &StripePath::default().param(id).param("refunds").param(fee), Self::object())
    }

    pub fn update<B: Serialize + StripeService>(client: &Client, id: &str, refund: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::ApplicationFees, &StripePath::default().param(id).param("refunds").param(refund), param)
    }

    pub fn list<B: Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::ApplicationFees, &StripePath::default().param(id), param)
    }

}