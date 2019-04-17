use crate::resources::common::object::Object;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct UsageRecord {
    pub id: String,
    pub object: Object,
    pub livemode: bool,
    pub quantity: i64,
    pub subscription_item: String,
    pub timestamp: i64
}

#[derive(Default, Serialize, Debug)]
pub struct UsageRecordParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<UsageAction>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum UsageAction {
    Increment,
    Set
}

impl StripeService for UsageRecord {}

impl UsageRecord {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionItems, &StripePath::default().param(id).param("usage_records"), param)
    }

}