use crate::client::Client;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::file::File;
use crate::util::List;

use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct ScheduledQueryRun {
    pub id: String,
    pub object: Object,
    pub created: i64,
    pub data_load_time: i64,
    pub error: Option<SigmaError>,
    pub file: Option<File>,
    pub livemode: bool,
    pub result_available_until: i64,
    pub sql: String,
    pub status: SigmaStatus,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct SigmaError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SigmaStatus {
    Completed,
    Canceled,
    Failed,
    TimedOut,
}

#[derive(Debug, Serialize)]
pub struct ScheduleQueryRunListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl ScheduledQueryRun {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::Sigma,
            vec![id],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Sigma, vec![], param)
    }
}
