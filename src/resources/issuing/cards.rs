use crate::resources::common::object::Object;
use crate::resources::paymentmethods::cards::CardBrand;
use crate::resources::issuing::cardholders::CardHolders;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::resources::common::category::MerchantCategories;
use crate::resources::common::address::Address;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
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
    pub shipping: IssuingShipping,
    pub status: CardStatus,
    #[serde(rename = "type")]
    pub card_type: IssuingCardType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationControls {
    pub allowed_categories: Option<Vec<MerchantCategories>>,
    pub blocked_categories: Option<Vec<MerchantCategories>>,
    pub currency: Currency,
    pub max_amount: i64,
    pub max_approvals: i64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum CardStatus {
    Active,
    Inactive,
    Pending,
    Canceled,
    Lost,
    Stolen
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum IssuingCardType {
    Virtual,
    Physical
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum IssuingShippingStatus {
    Pending,
    Shipped,
    Delivered,
    Returned,
    Failure,
    Canceled
}

#[derive(Serialize, Debug)]
pub struct IssuingCardParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<AuthorizationControls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<CardHolders>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
}

impl StripeService for IssuingCard {}
impl<'a> StripeService for IssuingCardParam<'a> {}

impl IssuingCard {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingCard, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::IssuingCard, &StripePath::default().param(id), Self::object())
    }

    pub fn retrieve_details(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::IssuingCard, &StripePath::default().param(id).param("details"), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::IssuingCard, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::IssuingCard, &StripePath::default(), param)
    }

}