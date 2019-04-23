use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::orders::sku::PackageDimensions;
use crate::util::{Deleted, List};
use crate::{Client};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Products {
    pub id: String,
    pub object: Object,
    pub active: bool,
    pub attributes: Vec<String>,
    pub caption: Option<String>,
    pub created: i64,
    pub deactivate_on: Vec<String>,
    pub description: Option<String>,
    pub images: Vec<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
    pub package_dimensions: Option<PackageDimensions>,
    pub shippable: Option<bool>,
    pub statement_descriptor: Option<String>,
    #[serde(rename = "type")]
    pub product_type: ProductType,
    pub unit_label: Option<String>,
    pub updated: i64,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    Service,
    Good,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ProductsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivate_on: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Products {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Products, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Products, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Products, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Products, vec![], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Products, vec![id], serde_json::Map::new())
    }
}
