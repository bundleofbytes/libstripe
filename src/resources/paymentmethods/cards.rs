use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List};
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub object: Object,
    pub account: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<CardCheck>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<CardCheck>,
    pub available_payout_methods: Option<Vec<AvailablePayoutMethods>>,
    pub brand: CardBrand,
    pub country: String,
    pub currency: Option<Currency>,
    pub customer: Option<String>,
    pub cvc_check: Option<CardCheck>,
    pub dynamic_last4: Option<String>,
    pub exp_month: i32,
    pub exp_year: i32,
    pub fingerprint: String,
    pub funding: CardType,
    pub last4: String,
    pub metadata: HashMap<String, String>,
    pub name: Option<String>,
    pub recipient: Option<String>,
    pub tokenization_method: Option<TokenizationMethod>,
}

#[derive(Default, Serialize, Debug)]
pub struct CardParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub enum AvailablePayoutMethods {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "instant")]
    Instant
}

//NOTE: Workaround to add an object name while leaving the rest "default"
impl<'a> CardParam<'a> {
    pub fn default() -> Self {
        CardParam {
            object: Some(Object::Card),
            ..Default::default()
        }
    }
}

#[derive(Default, Serialize, Debug)]
pub struct CardListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Unknown
}

//Doing a 'rename' just to insure that things will just work. can be removed
#[derive(Deserialize, PartialEq, Debug, Clone)]
pub enum CardBrand {
    #[serde(rename = "Visa")]
    Visa,
    #[serde(rename = "American Express")]
    AmericanExpress,
    #[serde(rename = "MasterCard")]
    MasterCard,
    #[serde(rename = "Discover")]
    Discover,
    #[serde(rename = "JCB")]
    JCB,
    #[serde(rename = "Diners Club")]
    DinersClub,
    #[serde(rename = "UnionPay")]
    UnionPay,
    #[serde(rename = "Unknown")]
    Unknown
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum CardCheck {
    Pass,
    Failed,
    Unavailable,
    Unchecked
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum TokenizationMethod {
    #[serde(rename = "apple_pay")]
    ApplePay,
    #[serde(rename = "android_pay")]
    AndroidPay
}

impl StripeService for Card {}
impl<'a> StripeService for CardParam<'a> {}
impl StripeService for CardListParams {}

impl Card {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources"), param)
    }

    pub fn retrieve(client: &Client, customer_id: &str, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), param)
    }

    pub fn delete(client: &Client, customer_id: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources"), param)
    }

}

