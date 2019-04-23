use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::product::Products;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub product: Expandable<Products>,
    pub updated: i64,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct PackageDimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64,
}

#[derive(Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct Inventory {
    pub quantity: Option<i32>,
    #[serde(rename = "type")]
    pub inventory_type: Option<InventoryType>,
    pub value: Option<InventoryValue>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InventoryValue {
    InStock,
    Limited,
    OutOfStock,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InventoryType {
    Finite,
    Bucket,
    Infinite,
}

#[derive(Default, Debug, Serialize, PartialEq)]
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
    pub package_dimensions: Option<PackageDimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct SkuListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_stock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Sku {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sku, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Sku, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Sku, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Sku, vec![], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Sku, vec![id], serde_json::Map::new())
    }
}
