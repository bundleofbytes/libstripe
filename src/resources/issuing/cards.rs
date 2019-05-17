use crate::resources::common::address::Address;
use crate::resources::common::category::MerchantCategories;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::issuing::cardholders::CardHolders;
use crate::resources::paymentmethods::cards::CardBrand;
use crate::util::{List, RangeQuery, Expandable};
use crate::Client;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IssuingCard {
    pub id: String,
    pub object: Object,
    pub authorization_controls: AuthorizationControls,
    pub brand: CardBrand,
    pub cardholder: CardHolders,
    pub created: i64,
    pub currency: Currency,
    pub exp_month: i64,
    pub exp_year: i64,
    pub last4: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub replacement_for: Option<Expandable<IssuingCard>>,
    pub replacement_reason: Option<String>,
    pub shipping: Option<IssuingShipping>,
    pub status: CardStatus,
    #[serde(rename = "type")]
    pub card_type: IssuingCardType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuthorizationControls {
    pub allowed_categories: Option<Vec<MerchantCategories>>,
    pub blocked_categories: Option<Vec<MerchantCategories>>,
    pub currency: Currency,
    pub max_amount: i64,
    pub max_approvals: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CardStatus {
    Active,
    Inactive,
    Pending,
    Canceled,
    Lost,
    Stolen,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssuingCardType {
    Virtual,
    Physical,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IssuingShipping {
    pub address: Address,
    pub carrier: String,
    pub eta: i64,
    pub name: String,
    pub phone: String,
    pub status: IssuingShippingStatus,
    pub tracking_number: String,
    pub tracking_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IssuingShippingStatus {
    Pending,
    Shipped,
    Delivered,
    Returned,
    Failure,
    Canceled,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct IssuingCardParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<AuthorizationControls>,
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub issuing_card_type: Option<CardHolders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<AuthorizationControls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<CardHolders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<IssuingShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct IssuingCardListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub issuing_card_type: Option<CardHolders>,
}

impl IssuingCard {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingCard, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::IssuingCard, vec![id], serde_json::Map::new())
    }

    pub fn retrieve_details(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::IssuingCard, vec![id, "details"], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingCard, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::IssuingCard, vec![], param)
    }
}
