use crate::resources::common::object::Object;
use crate::resources::core::charges::BillingDetails;
use crate::resources::paymentmethods::cards::{CardBrand, CardType, CardCheck};
use std::collections::HashMap;
use crate::resources::paymentmethods::bank::AccountHolderType;
use crate::resources::common::path::{StripePath, UrlPath};
use crate::resources::paymentmethods::source::PaymentSourceParam;
use crate::{StripeService, Client};
use crate::util::List;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct PaymentMethods {
    pub id: String,
    pub object: Object,
    pub billing_details: BillingDetails,
    pub card: PaymentCard,
    pub card_present: Option<String>,
    pub created: i64,
    pub customer: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    #[serde(rename="type")]
    pub payment_method_type: PaymentMethodsType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PaymentCard {
    pub brand: CardBrand,
    pub checks: ChargeChecks,
    pub country: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub fingerprint: String,
    pub funding: CardType,
    pub generated_from: Option<String>,
    pub last4: String,
    pub three_d_secure_usage: ThreeDSecureUsage,
    pub wallet: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ThreeDSecureUsage {
    pub supported: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct GenerateFrom {
    pub charge: String,
    pub payment_method_details: PaymentMethodsDetails
}

#[derive(Debug, Deserialize, Clone)]
#[serde(remote = "Self", rename_all="snake_case")]
pub enum PaymentMethodsDetails {
    AchCreditTransfer { account_number: String, bank_name: String, routing_number: String, swift_code: String },
    AchDebit { account_holder_type: AccountHolderType,  bank_name: String, country: String, fingerprint: String, last4: String, routing_number: String },
    AliPay,
    BanContact { bank_code: String, bank_name: String, bic: String, iban_last4: String, preferred_language: String, verified_name: String },
    Card { brand: String, checks: ChargeChecks, country: String, exp_month: i32, exp_year: i32, fingerprint: String, funding: CardType, last4: String, three_d_secure: Option<String>, wallet: Option<String> },
    //TODO: Complete
}

impl<'de> Deserialize<'de> for PaymentMethodsDetails {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Inner {
            #[serde(rename = "type")]
            _type: String,
            #[serde(flatten, with = "PaymentMethodsDetails")]
            inner: PaymentMethodsDetails,
        }
        Inner::deserialize(deserializer).map(|w| w.inner)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChargeChecks {
    pub address_line1_check: Option<CardCheck>,
    pub address_postal_code_check: Option<CardCheck>,
    pub cvc_check: Option<CardCheck>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="snake_case")]
pub enum PaymentMethodsType {
    Card,
    CardPresent,
}

#[derive(Serialize, Debug)]
pub struct PaymentMethodsParam<'a> {
    #[serde(rename="type")]
    pub payment_method_type: Option<PaymentMethodsType>,
    pub billing_details: Option<BillingDetails>,
    pub card: Option<PaymentSourceParam<'a>>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentMethodsListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl StripeService for PaymentMethods {}
impl<'a> StripeService for PaymentMethodsParam<'a> {}
impl StripeService for PaymentMethodsListParams {}

impl PaymentMethods {
    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::PaymentMethods, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client,  param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::PaymentMethods, &StripePath::default(), param)
    }

    pub fn attach<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, &StripePath::default().param(id).param("attach"), param)
    }

    pub fn detach<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, &StripePath::default().param(id).param("attach"), param)
    }

}