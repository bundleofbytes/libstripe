use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::util::{Expandable, List};
use crate::resources::core::customer::Customer;
use crate::resources::billing::invoices::Invoice;
use crate::resources::core::refunds::Refund;
use crate::resources::common::path::UrlPath;
use crate::Client;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CreditNotes {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub created: i64,
    pub currency: Currency,
    pub customer: Expandable<Customer>,
    pub invoice: Expandable<Invoice>,
    pub livemode: bool,
    pub memo: Option<String>,
    pub metadata: HashMap<String, String>,
    pub number: String,
    pub pdf: String,
    pub reason: Option<CreditNoteReason>,
    pub refund: Option<Expandable<Refund>>,
    pub status: CreditNoteStatus,
    #[serde(rename="type")]
    pub credit_note_type: CreditNoteType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum CreditNoteStatus {
    Issued,
    Void
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum CreditNoteType {
    PostPayment,
    PrePayment
}

#[derive(Serialize, Default, Debug, PartialEq)]
pub struct CreditNoteParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<CreditNoteReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Serialize, Default, Debug, PartialEq)]
pub struct CreditNoteListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl CreditNotes {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::CreditNote, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::CreditNote, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::CreditNote, vec![id], param)
    }

    pub fn void<B: serde::Serialize>(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::CreditNote, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::CreditNote, vec![], param)
    }

}