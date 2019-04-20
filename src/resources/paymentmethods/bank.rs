use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List};
use crate::Client;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AccountHolderType {
    Individual,
    Company,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BankStatus {
    New,
    Validated,
    Verified,
    VerificationFailed,
    Errored,
}

impl BankAccount {
    pub fn create<B: serde::Serialize>(
        client: &Client,
        customer_id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![customer_id, "sources"], param)
    }

    pub fn retrieve(client: &Client, customer_id: &str, id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::Customers,
            vec![customer_id, "sources", id],
            serde_json::Map::new(),
        )
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        customer_id: &str,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![customer_id, "sources", id], param)
    }

    pub fn verify<B: serde::Serialize>(
        client: &Client,
        customer_id: &str,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(
            UrlPath::Customers,
            vec![customer_id, "sources", id, "verify"],
            param,
        )
    }

    pub fn delete(client: &Client, customer_id: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(
            UrlPath::Customers,
            vec![customer_id, "sources", id],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: serde::Serialize>(
        client: &Client,
        customer_id: &str,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(UrlPath::Customers, vec![customer_id, "sources"], param)
    }
}
