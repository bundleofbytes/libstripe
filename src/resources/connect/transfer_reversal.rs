use crate::resources::common::object::Object;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct TransferReversal {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: Option<BalanceTransaction>,
    pub created: i64,
    pub currency: Currency,
    pub destination_payment_refund: Option<String>,
    pub metadata: HashMap<String, String>,
    pub source_refund: Option<String>,
    pub transfer: String
}

#[derive(Default, Serialize, Debug)]
pub struct TransferReveralParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct TransferReversalListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl StripeService for TransferReversal {}
impl<'a> StripeService for TransferReveralParam<'a> {}
impl<'a> StripeService for TransferReversalListParams<'a> {}

impl TransferReversal {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transfers, &StripePath::default().param(id).param("reversals"), param)
    }

    pub fn retrieve(client: &Client, id: &str, reversal_id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Transfers, &StripePath::default().param(id).param("reversals").param(reversal_id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, reversal_id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Transfers, &StripePath::default().param(id).param("reversals").param(reversal_id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Transfers, &StripePath::default().param(id).param("reversals"), param)
    }

}