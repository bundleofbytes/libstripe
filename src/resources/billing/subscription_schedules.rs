use crate::resources::billing::subscriptions::{BillingThresholds, SubscriptionBilling};
use crate::resources::common::object::Object;
use crate::resources::common::path::{UrlPath};
use crate::util::{List, Expandable};
use crate::Client;
use std::collections::HashMap;
use crate::resources::core::customer::Customer;
use crate::resources::billing::coupons::Coupon;
use crate::resources::billing::plans::Plans;
use crate::resources::billing::invoices::InvoiceBilling;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SubscriptionSchedules {
    pub id: String,
    pub object: Object,
    pub billing: InvoiceBilling,
    pub billing_thresholds: Option<BillingThresholds>,
    pub canceled_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub created: i64,
    pub current_phase: Option<CurrentPhase>,
    pub customer: Expandable<Customer>,
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
    pub subscription: Option<String>, //Expandable?
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InvoiceSettings {
    pub days_until_due: i64,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct Phase {
    pub application_fee_percent: Option<f64>,
    pub coupon: Option<Expandable<Coupon>>,
    pub end_date: Option<i64>,
    pub start_date: Option<i64>,
    pub trail_end: Option<i64>,
    pub trail: Option<bool>,
    pub tax_percent: Option<f64>,
    pub plans: Option<Vec<PhasePlans>>,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct PhasePlans {
    pub billing_thresholds: Option<BillingThresholds>,
    pub plan: Option<Expandable<Plans>>,
    pub quantity: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CurrentPhase {
    pub start_date: i64,
    pub end_date: i64,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct RenewalInterval {
    pub interval: Option<String>,
    pub length: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulesStatus {
    NotStarted,
    Active,
    Completed,
    Released,
    Canceled,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RenewalBehavior {
    None,
    Renew,
    Release,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub preserve_cancel_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl SubscriptionSchedules {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::SubscriptionSchedules,
            vec![id],
            serde_json::Map::new(),
        )
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, vec![id], param)
    }

    pub fn cancel<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::SubscriptionSchedules, vec![id, "cancel"], param)
    }

    pub fn release<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionSchedules, vec![id, "release"], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::SubscriptionSchedules, vec![], param)
    }
}
