use crate::resources::billing::plans::Plans;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List, Period, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::customer::Customer;
use crate::resources::billing::invoices::Invoice;
use crate::resources::billing::subscriptions::Subscription;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceItems {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub currency: Currency,
    pub customer: Expandable<Customer>,
    pub date: i64,
    pub description: Option<String>,
    pub discountable: bool,
    pub invoice: Option<Expandable<Invoice>>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub period: Period,
    pub plan: Option<Plans>,
    pub proration: bool,
    pub quantity: Option<i64>,
    pub subscription: Option<Expandable<Subscription>>,
    pub subscription_item: Option<String>,
}

#[derive(Default, Serialize, Debug)]
pub struct InvoiceItemsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug)]
pub struct InvoiceItemsListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl InvoiceItems {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::InvoiceItems, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::InvoiceItems, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::InvoiceItems, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::InvoiceItems, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::InvoiceItems, vec![], param)
    }
}
