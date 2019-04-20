use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::connect::account::{BusinessType, CompanyParam};
use crate::resources::connect::persons::PersonsParam;
use crate::resources::paymentmethods::bank::{BankAccount, BankAccountParam};
use crate::resources::paymentmethods::cards::{Card, CardParam};
use crate::{Client};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub id: String,
    pub object: Object,
    pub card: Option<Card>,
    pub bank_account: Option<BankAccount>,
    pub client_ip: Option<String>,
    pub created: i64,
    pub livemode: bool,
    #[serde(rename = "type")]
    pub token_type: TokenType,
    pub used: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Card,
    BankAccount,
}

#[derive(Default, Serialize, Debug)]
pub struct TokenParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccountParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Option<PIIParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountTokenParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct AccountTokenParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonsParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct PIIParam<'a> {
    pub id_number: &'a str,
}

impl Tokens {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Tokens, vec![], param)
    }

    pub fn retrieve(client: &Client, token: &str) -> crate::Result<Self> {
        client.get(UrlPath::Tokens, vec![token], serde_json::Map::new())
    }
}
