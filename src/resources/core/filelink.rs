use crate::resources::common::object::Object;
use std::collections::HashMap;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::resources::common::path::StripePath;

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
    pub url: String
}

#[derive(Debug, Serialize)]
pub struct FileLinkParam<'a> {
    pub file: Option<&'a str>,
    pub expires_at: Option<i64>,
    pub metadata: Option<HashMap<&'a str, &'a str>>
}

impl StripeService for FileLink {}

impl FileLink {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::FileLink, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, link: &str) -> crate::Result<Self> {
        client.get(UrlPath::FileLink, &StripePath::default().param(link), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::FileLink, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::FileLink, &StripePath::default(), param)
    }
}