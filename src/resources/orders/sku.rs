use crate::resources::common::object::Object;
use std::collections::HashMap;
use crate::resources::common::currency::Currency;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::{List, Deleted};
use crate::resources::common::path::StripePath;

#[derive(Debug, Deserialize)]
pub struct Sku {
    pub id: String,
    pub object: Object,
    pub active: bool,
    pub attributes: Option<HashMap<String, String>>,
    pub created: i64,
    pub currency: Currency,
    pub image: Option<String>,
    pub inventory: Inventory,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub package_dimensions: Option<PackageDimensions>,
    pub price: i64,
    pub product: String,
    pub updated: i64
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Inventory {
    pub quantity: Option<i32>,
    #[serde(rename="type")]
    pub inventory_type: Option<InventoryType>,
    pub value: Option<InventoryValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="snake_case")]
pub enum InventoryValue {
    InStock,
    Limited,
    OutOfStock,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="lowercase")]
pub enum InventoryType {
    Finite,
    Bucket,
    Infinite
}

#[derive(Default, Debug, Serialize)]
pub struct SkuParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory: Option<Inventory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>
}

impl StripeService for Sku {}

impl<'a> StripeService for SkuParam<'a> {}


impl Sku {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sku, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Sku, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sku, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Sku, &StripePath::default(), param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Sku, &StripePath::default().param(id), Self::object())
    }

}

