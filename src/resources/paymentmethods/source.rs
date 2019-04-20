use crate::resources::common::address::Address;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::refunds::FailureReason;
use crate::resources::paymentmethods::bank::{BankAccount, BankAccountParam};
use crate::resources::paymentmethods::cards::{Card, CardParam};
use crate::{Client};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub id: String,
    pub object: Object,
    pub ach_credit_transfer: Option<AchCreditTransfer>,
    pub amount: i32,
    pub client_secret: String,
    pub code_verification: CodeVerification,
    pub created: i64,
    pub currency: Currency,
    pub customer: String,
    pub flow: SourceFlow,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub owner: SourceOwner,
    pub receiver: Option<SourceReceiver>,
    pub redirect: Option<SourceRedirect>,
    pub statement_descriptor: Option<String>,
    pub status: SourceStatus,
    #[serde(rename = "type")]
    pub source_type: SourceType,
    pub usage: SourceUsage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AchCreditTransfer {
    pub account_number: String,
    pub routing_number: String,
    pub fingerprint: String,
    pub bank_name: String,
    pub swift_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeVerification {
    pub attempts_remaining: i64,
    pub status: VerificationStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceRedirect {
    pub failure_reason: FailureReason,
    pub return_url: String,
    pub status: SourceRedirectStatus,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RedirectFailureReason {
    UserAbort,
    Declined,
    ProcessingError,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SourceRedirectStatus {
    Pending,
    Succeeded,
    NotRequired,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceOwner {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub verified_address: Option<Address>,
    pub verified_email: Option<String>,
    pub verified_name: Option<String>,
    pub verified_phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceReceiver {
    pub address: String,
    pub amount_charged: i32,
    pub amount_received: i32,
    pub amount_returned: i32,
    pub refund_attributes_method: String,
    pub refund_attributes_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SourceFlow {
    Redirect,
    Receiver,
    CodeVerification,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SourceUsage {
    Reusable,
    SingleUse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SourceStatus {
    Canceled,
    Chargeable,
    Consumed,
    Failed,
    Pending,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    Alypay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giopay,
    Ideal,
    Multibanco,
    P24,
    PaperCheck,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PaymentSource {
    BankAccount(BankAccount),
    Card(Card),
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum PaymentSourceParam<'a> {
    BankAccount(BankAccountParam<'a>),
    Card(CardParam<'a>),
    Token(&'a str),
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Serialize, Debug)]
pub struct SourceParam<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<SourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<SourceFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<SourceOwner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<SourceRedirectParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<SourceUsage>,
}

#[derive(Default, Serialize, Debug)]
pub struct SourceRedirectParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
}

impl Source {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sources, vec![], param)
    }

    pub fn retrieve<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.get(UrlPath::Sources, vec![id], param)
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sources, vec![id], param)
    }

    //TODO: Review documents
    pub fn attach<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![id, "sources"], param)
    }

    pub fn detach<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![id, "sources"], param)
    }
}
