use crate::resources::common::object::Object;
use crate::resources::common::path::{UrlPath};
use crate::resources::core::events::EventType;
use crate::util::{Deleted, List};
use crate::{Client};
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookEndpoints {
    pub id: String,
    pub object: Object,
    pub api_version: String,
    pub application: String,
    pub created: i64,
    pub enabled_events: Vec<EventType>,
    pub livemode: bool,
    pub status: EndpointStatus,
    pub url: String,
    pub secret: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum EndpointStatus {
    Enabled,
    Disabled,
}

#[derive(Serialize, Debug)]
pub struct WebhookEndpointsParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_events: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct WebhookEndpointsListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl WebhookEndpoints {
    pub fn create<S: Serialize>(client: &Client, param: S) -> crate::Result<Self> {
        client.post(UrlPath::WebhookEndpoints, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::WebhookEndpoints, vec![id], serde_json::Map::new())
    }

    pub fn update<S: Serialize>(client: &Client, id: &str, param: S) -> crate::Result<Self> {
        client.post(UrlPath::WebhookEndpoints, vec![id], param)
    }

    pub fn list<S: Serialize>(client: &Client, param: S) -> crate::Result<List<Self>> {
        client.get(UrlPath::WebhookEndpoints, vec![], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.get(UrlPath::WebhookEndpoints, vec![id], serde_json::Map::new())
    }
}
