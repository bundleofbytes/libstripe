use crate::resources::common::object::Object;
use crate::resources::common::path::{UrlPath};
use crate::resources::core::charges::BillingDetails;
use crate::resources::paymentmethods::bank::AccountHolderType;
use crate::resources::paymentmethods::cards::{CardBrand, CardCheck, CardType};
use crate::resources::paymentmethods::source::PaymentSourceParam;
use crate::util::List;
use crate::{Client};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(rename = "type")]
    pub payment_method_type: PaymentMethodsType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentCard {
    pub brand: CardBrand,
    pub checks: ChargeChecks,
    pub country: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub fingerprint: String,
    pub funding: CardType,
    pub generated_from: Option<GenerateFrom>,
    pub last4: String,
    pub three_d_secure_usage: ThreeDSecureUsage,
    pub wallet: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateFrom {
    pub charge: String,
    pub payment_method_details: PaymentMethodsDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodsDetails {
    #[serde(rename = "type")]
    pub method_type: String,
    #[serde(flatten, with = "PaymentMethodsDetailsInner")]
    pub method: PaymentMethodsDetailsInner,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "Self", rename_all = "snake_case")]
pub enum PaymentMethodsDetailsInner {
    AchCreditTransfer {
        account_number: String,
        bank_name: String,
        routing_number: String,
        swift_code: String,
    },
    AchDebit {
        account_holder_type: AccountHolderType,
        bank_name: String,
        country: String,
        fingerprint: String,
        last4: String,
        routing_number: String,
    },
    AliPay,
    BanContact {
        bank_code: String,
        bank_name: String,
        bic: String,
        iban_last4: String,
        preferred_language: String,
        verified_name: String,
    },
    Card {
        brand: String,
        checks: ChargeChecks,
        country: String,
        exp_month: i32,
        exp_year: i32,
        fingerprint: String,
        funding: CardType,
        last4: String,
        three_d_secure: Option<String>,
        wallet: Option<String>,
    },
    //TODO: Complete
}

//TODO: Relook at this in the future before 0.6.0 release
//impl<'de> Deserialize<'de> for PaymentMethodsDetails {
//    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
//        #[derive(Deserialize)]
//        struct Inner {
//            #[serde(rename = "type")]
//            _type: String,
//            #[serde(flatten, with = "PaymentMethodsDetails")]
//            inner: PaymentMethodsDetails,
//        }
//        Inner::deserialize(deserializer).map(|w| w.inner)
//    }
//}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChargeChecks {
    pub address_line1_check: Option<CardCheck>,
    pub address_postal_code_check: Option<CardCheck>,
    pub cvc_check: Option<CardCheck>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodsType {
    Card,
    CardPresent,
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentMethodsParam<'a> {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<BillingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentMethodsListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl PaymentMethods {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::PaymentMethods, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::PaymentMethods, vec![], param)
    }

    pub fn attach<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, vec![id, "attach"], param)
    }

    pub fn detach<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentMethods, vec![id, "attach"], param)
    }
}
