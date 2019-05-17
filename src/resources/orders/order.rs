use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::charges::{ShippingDetails, Charge};
use crate::util::{List, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::orders::sku::Sku;
use crate::resources::billing::discounts::Discount;
use crate::resources::core::customer::Customer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Order {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub amount_returned: Option<i64>,
    pub application: Option<String>,
    pub application_fee: Option<i64>,
    pub charge: Option<Expandable<Charge>>,
    pub created: i64,
    pub currency: Currency,
    pub customer: Option<Expandable<Customer>>,
    pub email: Option<String>,
    pub items: Vec<OrderItem>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub returns: List<OrderReturn>,
    pub selected_shipping_method: Option<String>,
    pub shipping: ShippingDetails,
    pub shipping_methods: Vec<ShippingMethods>,
    pub status: OrderStatus,
    pub status_transitions: OrderTransitions,
    pub updated: i64,
    pub upstream_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DeliveryEstimate {
    #[serde(rename = "type")]
    pub delivery_type: DeliveryType,
    pub latest: Option<String>,
    pub earliest: Option<String>,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeliveryType {
    Range,
    Exact,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShippingMethods {
    pub id: String,
    pub amount: i64,
    pub currency: Currency,
    pub delivery_estimate: Option<DeliveryEstimate>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderTransitions {
    pub canceled: i64,
    pub fulfiled: Option<i64>,
    pub paid: i64,
    pub returned: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderReturn {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub created: i64,
    pub currency: Currency,
    pub items: Vec<OrderItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderItem {
    pub object: Option<Object>,
    pub amount: Option<i64>,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    pub parent: Option<Expandable<OrderItemParent>>,
    pub quantity: Option<i64>,
    #[serde(rename = "type")]
    pub item_type: Option<ItemType>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum OrderItemParent {
    Sku(Sku),
    Discount(Discount)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Sku,
    Tax,
    Shipping,
    Discount,
    #[serde(other, skip_serializing)]
    Unknown,
}

impl Default for ItemType {
    fn default() -> ItemType {
        ItemType::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Created,
    Paid,
    Canceled,
    Fulfilled,
    Returned,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct OrderParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<OrderItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Order {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Order, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Order, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Order, vec![id], param)
    }

    pub fn pay<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Order, vec![id, "pay"], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Order, vec![], param)
    }

    pub fn return_item<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Order, vec![id, "returns"], param)
    }
}
