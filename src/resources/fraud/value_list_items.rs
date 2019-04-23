use crate::resources::common::object::Object;
use crate::util::{RangeQuery, Deleted, List};
use crate::Client;
use crate::resources::common::path::UrlPath;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ValueListItems {
    pub id: String,
    pub object: Object,
    pub created: i64,
    pub created_by: String,
    pub livemode: bool,
    pub value: String,
    pub value_list: String,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct ValueListItemsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<&'a str>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ValueListItemsListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<&'a str>,
}

impl ValueListItems {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::RadarValueListItems, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::RadarValueListItems, vec![id], serde_json::Map::new())
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::RadarValueListItems, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::RadarValueListItems, vec![], param)
    }

}