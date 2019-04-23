use crate::resources::common::object::Object;
use crate::util::{List, Deleted, RangeQuery};
use std::collections::HashMap;
use crate::resources::fraud::value_list_items::ValueListItems;
use crate::Client;
use crate::resources::common::path::UrlPath;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ValueLists {
    pub id: String,
    pub object: Object,
    pub alais: String,
    pub created: i64,
    pub created_by: String,
    pub item_type: ItemType,
    pub list_items: List<ValueListItems>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum ItemType {
    CardFingerprint,
    CardBin,
    Email,
    IpAddress,
    Country,
    String,
    CaseSensitiveString
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ValueListsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<ItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ValueListsListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
}

impl ValueLists {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::RadarValueList, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::RadarValueList, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::RadarValueList, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::RadarValueList, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::RadarValueList, vec![], param)
    }
}