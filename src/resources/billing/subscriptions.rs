use crate::resources::common::object::Object;
use crate::resources::billing::discounts::Discount;
use crate::util::List;
use std::collections::HashMap;
use crate::resources::billing::plans::Plans;
use crate::resources::issuing::cardholders::Billing;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::paymentmethods::source::PaymentSourceParam;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct Subscription {
    pub id: String,
    pub object: Object,
    pub application_fee_percent: Option<i32>,
    pub billing: SubscriptionBilling,
    pub billing_cycle_anchor: i64,
    pub billing_thresholds: Option<BillingThresholds>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub created: i64,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub customer: String,
    pub days_until_due: Option<i64>,
    pub default_source: Option<String>,
    pub discount: Option<Discount>, //TODO: import
    pub ended_at: Option<i64>,
    pub items: List<SubscriptionItems>,
    pub livemode: bool,
    pub latest_invoice: String,
    pub metadata: HashMap<String, String>,
    pub plan: Plans,
    pub quantity: i64,
    pub start: i64,
    pub status: SubscriptionStatus,
    pub tax_percent: Option<f64>,
    pub trial_end: Option<i64>,
    pub trial_start: Option<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum SubscriptionBilling {
    ChargeAutomatically,
    SendInvoice
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BillingThresholds {
    pub amount_gte: i64,
    pub reset_billing_cycle_anchor: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionItems {
    pub id: String,
    pub object: Object,
    pub billing_thesholds: Option<BillingThresholds>,
    pub created: i64,
    pub metadata: HashMap<String, String>,
    pub plan: Plans,
    pub quantity: i64,
    pub subscription: String,
}

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
pub struct CurrentPhase {
    pub start_date: i64,
    pub end_date: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Phase {
    pub applicaiton_fee_percent: Option<f64>,
    pub coupon: Option<String>,
    pub end_date: i64,
    pub start_date: i64,
    pub trail_end: i64,
    pub trail: bool,
    pub tax_percent: Option<f64>,
    pub plans: Vec<PhasePlans>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhasePlans {
    pub billing_thresholds: Option<BillingThresholds>,
    pub plan: String,
    pub quantity: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceSettings {
    pub days_until_due: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenewalInterval {
    pub interval: String,
    pub length: i64
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Incomplete,
    IncompleteExpired,
}

#[derive(Default, Serialize, Debug)]
pub struct SubscriptionItemParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>
}

impl StripeService for SubscriptionItems {}
impl<'a> StripeService for SubscriptionItemParam<'a> {}

impl SubscriptionItems  {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionItems, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::SubscriptionItems, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionItems, &StripePath::default().param(id), param)
    }

    pub fn delete<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::SubscriptionItems, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::SubscriptionItems, &StripePath::default(), param)
    }

}

#[derive(Default, Serialize, Debug)]
pub struct SubscriptionParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ItemParam<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_period_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

#[derive(Default, Serialize, Debug)]
pub struct ItemParam<'a> {
    pub plan: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>
}

impl StripeService for Subscription {}
impl<'a> StripeService for SubscriptionParam<'a> {}
impl<'a> StripeService for ItemParam<'a> {}


impl Subscription {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Subscriptions, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Subscriptions, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Subscriptions, &StripePath::default().param(id), param)
    }

    pub fn cancel<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::Subscriptions, &StripePath::default().param(id), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Subscriptions, &StripePath::default(), param)
    }
}