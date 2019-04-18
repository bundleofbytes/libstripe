use crate::resources::common::object::Object;
use crate::resources::issuing::cardholders::Billing;
use std::collections::HashMap;
use crate::resources::billing::subscriptions::{BillingThresholds, SubscriptionBilling};
use crate::{StripeService, Client};
use crate::resources::common::path::{UrlPath, StripePath};
use crate::util::List;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionSchedules {
    pub id: String,
    pub object: Object,
    pub billing: Billing,
    pub billing_thresholds: Option<BillingThresholds>,
    pub canceled_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub created: i64,
    pub current_phase: Option<CurrentPhase>,
    pub customer: String,
    pub invoice_settings: Option<InvoiceSettings>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub phases: Vec<Phase>,
    pub released_at: Option<i64>,
    pub released_subscription: Option<String>,
    pub renewal_behavior: String,
    pub renewal_interval: Option<RenewalInterval>,
    pub revision: String,
    pub status: SubscriptionSchedulesStatus,
    pub subscription: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceSettings {
    pub days_until_due: i64
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Phase {
    pub applicaiton_fee_percent: Option<f64>,
    pub coupon: Option<String>,
    pub end_date: Option<i64>,
    pub start_date: Option<i64>,
    pub trail_end: Option<i64>,
    pub trail: Option<bool>,
    pub tax_percent: Option<f64>,
    pub plans: Option<Vec<PhasePlans>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PhasePlans {
    pub billing_thresholds: Option<BillingThresholds>,
    pub plan: Option<String>,
    pub quantity: Option<i64>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentPhase {
    pub start_date: i64,
    pub end_date: i64
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct RenewalInterval {
    pub interval: Option<String>,
    pub length: Option<i64>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum SubscriptionSchedulesStatus {
    NotStarted,
    Active,
    Completed,
    Released,
    Canceled
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum RenewalBehavior {
    None,
    Renew,
    Release
}

#[derive(Default, Serialize, Debug)]
pub struct SubscriptionSchedulesParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_threshold: Option<BillingThresholds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<Phase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_behavior: Option<RenewalBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_interval: Option<RenewalInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_cancel_date: Option<i64>
}

impl StripeService for SubscriptionSchedules {}
impl<'a> StripeService for SubscriptionSchedulesParam<'a> {}


impl SubscriptionSchedules {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::SubscriptionSchedules, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, &StripePath::default().param(id), param)
    }

    pub fn cancel<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::SubscriptionSchedules, &StripePath::default().param(id).param("cancel"), param)
    }

    pub fn release<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, &StripePath::default().param(id).param("release"), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::SubscriptionSchedules, &StripePath::default(), param)
    }
}