use crate::resources::common::object::Object;
use crate::resources::common::address::Address;
use crate::resources::common::path::UrlPath;
use crate::Client;
use crate::util::{Deleted, List};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Locations {
    pub id: String,
    pub object: Object,
    pub address: Address,
    pub display_name: String,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct LocationsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<&'a str>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct LocationsListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<&'a str>,
}

impl Locations {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TerminalLocations, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::TerminalLocations, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TerminalLocations, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::TerminalLocations, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::TerminalLocations, vec![], param)
    }

}