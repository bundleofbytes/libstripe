use crate::StripeService;
use crate::resources::common::object::Object;
use serde_json::Value;
use crate::client::Client;
use crate::resources::common::path::UrlPath;
use serde::Serialize;
use crate::util::List;
use crate::resources::common::path::StripePath;
use crate::resources::core::file::File;

#[derive(Debug, Deserialize)]
pub struct ScheduledQueryRun {
    pub id: String,
    pub object: Object,
    pub created: i64,
    pub data_load_time: i64,
    pub error: Option<Value>, //TODO
    pub file: Option<File>,
    pub livemode: bool,
    pub result_available_until: i64,
    pub sql: String,
    pub status: SigmaStatus,
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SigmaStatus {
    Completed,
    Canceled,
    Failed,
    TimedOut
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

impl StripeService for ScheduledQueryRun {}
impl<'a> StripeService for ScheduleQueryRunListParam<'a> {}

impl ScheduledQueryRun {

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Sigma, &StripePath::default().param("scheduled_query_run").param(id), Self::object())
    }

    pub fn list<B: Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Sigma, &StripePath::default().param("scheduled_query_run"), param)
    }

}