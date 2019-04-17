use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List};
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct BankAccount {
    pub id: String,
    pub object: Object,
    pub account: Option<String>,
    pub account_holder_name: String,
    pub account_holder_type: AccountHolderType,
    pub bank_name: String,
    pub country: String,
    pub customer: Option<String>,
    pub currency: Currency,
    pub default_for_currency: Option<bool>,
    pub fingerprint: String, 
    pub last4: String,
    pub metadata: HashMap<String, String>,
    pub routing_number: String,
    pub status: BankStatus,
}

#[derive(Default, Serialize, Debug)]
pub struct BankAccountParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<Object>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<AccountHolderType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

//NOTE: Workaround to add an object name while leaving the rest "default"
impl<'a> BankAccountParam<'a> {
    pub fn default() -> Self {
        BankAccountParam {
            object: Some(Object::BankAccount),
            ..Default::default()
        }
    }
}

#[derive(Default, Serialize, Debug)]
pub struct BankAccountVerifyParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amounts: Option<(u32, u32)>,
}

#[derive(Serialize, Debug)]
pub struct BankListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum AccountHolderType {
    Individual,
    Company
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all="snake_case")]
pub enum BankStatus {
    New,
    Validated,
    Verified,
    VerificationFailed,
    Errored
}

impl StripeService for BankAccount {}
impl<'a> StripeService for BankAccountParam<'a> {}
impl StripeService for BankAccountVerifyParam {}
impl<'a> StripeService for BankListParams<'a> {}

impl BankAccount {
    
    pub fn create<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources"), param)
    }

    pub fn retrieve(client: &Client, customer_id: &str, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), param)
    }

    pub fn verify<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id).param("verify"), param)
    }

    pub fn delete(client: &Client, customer_id: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources").param(id), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, customer_id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Customers, &StripePath::default().param(customer_id).param("sources"), param)
    }
    
}