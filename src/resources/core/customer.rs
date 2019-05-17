use crate::resources::billing::subscriptions::Subscription;
use crate::resources::common::address::Address;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;
use crate::resources::common::path::{UrlPath};
use crate::resources::paymentmethods::source::{PaymentSource, PaymentSourceParam, Source};
use crate::util::{Deleted, List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::customer_taxid::{CustomerTaxID, CustomerTaxIDParam};
use crate::resources::billing::discounts::Discount;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CustomerShipping {
    pub address: Address,
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Customer {
    pub id: String,
    pub object: Object,
    pub account_balance: i64,
    pub address: Option<Address>,
    pub created: i64,
    pub currency: Option<Currency>,
    pub default_source: Option<Expandable<Source>>,
    pub delinquent: bool,
    pub description: Option<String>,
    pub discount: Option<Expandable<Discount>>,
    pub email: Option<String>,
    pub invoice_prefix: Option<String>,
    pub invoice_settings: InvoiceSettings,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub preferred_locales: Option<Vec<String>>,
    pub shipping: Option<CustomerShipping>,
    pub sources: Option<List<PaymentSource>>,
    pub subscription: Option<List<Subscription>>,
    pub tax_exempt: Option<TaxExempt>,
    pub tax_ids: Option<List<CustomerTaxID>>,
    pub tax_info: Option<TaxInfo>, //Deprecated
    pub tax_info_verification: Option<TaxInfoVerification>, //Deprecated
}

//TODO: Move to invoice?
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InvoiceSettings {
    pub custom_fields: Option<CustomFields>,
    pub default_payment_method: Option<String>,
    pub footer: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CustomFields {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum TaxExempt {
    None,
    Exempt,
    Reversed
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TaxInfo {
    pub tax_id: String,
    #[serde(rename = "type")]
    pub tax_type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TaxInfoVerification {
    pub verified_name: String,
    pub status: TaxStatus,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    Unverified,
    Pending,
    Verified,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct CustomerParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CustomerShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<TaxInfo>, //Deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<TaxExempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<CustomerTaxIDParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
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
