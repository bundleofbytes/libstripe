use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::{Client};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FileLink {
    pub id: String,
    pub object: Object,
    pub created: i64,
    pub expired: bool,
    pub expires_at: Option<i64>,
    pub file: String,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct FileLinkParam<'a> {
    pub file: Option<&'a str>,
    pub expires_at: Option<i64>,
    pub metadata: Option<HashMap<&'a str, &'a str>>,
}

impl FileLink {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::FileLink, vec![], param)
    }

    pub fn retrieve(client: &Client, link: &str) -> crate::Result<Self> {
        client.get(UrlPath::FileLink, vec![link], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::FileLink, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::FileLink, vec![], param)
    }
}
