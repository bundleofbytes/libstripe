use crate::resources::common::object::Object;
use crate::Client;
use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List};

#[derive(Deserialize, Debug)]
pub struct Readers {
    pub id: String,
    pub object: Object,
    pub device_sw_version: Option<String>,
    pub device_type: String,
    pub ip_address: String,
    pub label: String,
    pub location: Option<String>,
    pub serial_number: String,
    pub status: ReadersStatus
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="lowercase")]
pub enum ReadersStatus {
    Online,
    Offline
}

#[derive(Default, Debug, Serialize)]
pub struct ReadersParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<&'a str>,
}

#[derive(Default, Serialize, Debug)]
pub struct ReadersListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

impl Readers {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TerminalReaders, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::TerminalReaders, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::TerminalReaders, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::TerminalReaders, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::TerminalReaders, vec![], param)
    }

}