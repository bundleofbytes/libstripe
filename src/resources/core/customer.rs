use crate::resources::billing::subscriptions::Subscription;
use crate::resources::common::address::Address;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;
use crate::resources::common::path::{UrlPath};
use crate::resources::paymentmethods::source::{PaymentSource, PaymentSourceParam};
use crate::util::{Deleted, List, RangeQuery};
use crate::{Client};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerShipping {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Deserialize, Debug)]
pub struct Customer {
    pub id: String,
    pub object: Object,
    pub account_balance: i64,
    pub created: i64,
    pub currency: Option<Currency>,
    pub default_source: Option<String>,
    pub delinquent: bool,
    pub description: Option<String>,
    pub discount: Option<String>,
    pub email: Option<String>,
    pub invoice_prefix: Option<String>,
    pub invoice_settings: InvoiceSettings,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub shipping: Option<CustomerShipping>,
    pub sources: Option<List<PaymentSource>>,
    pub subscription: Option<List<Subscription>>,
    pub tax_info: Option<TaxInfo>,
    pub tax_info_verification: Option<TaxInfoVerification>,
}

//TODO: Move to invoice?
#[derive(Deserialize, Debug)]
pub struct InvoiceSettings {
    pub custom_fields: Option<CustomFields>,
    pub footer: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CustomFields {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaxInfo {
    pub tax_id: String,
    #[serde(rename = "type")]
    pub tax_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TaxInfoVerification {
    pub verified_name: String,
    pub status: TaxStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    Unverified,
    Pending,
    Verified,
}

#[derive(Default, Debug, Serialize)]
pub struct CustomerParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_vat_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<TaxInfo>,
}

#[derive(Default, Serialize, Debug)]
pub struct CustomerListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl Customer {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Customers, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Customers, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Customers, vec![], param)
    }
}
