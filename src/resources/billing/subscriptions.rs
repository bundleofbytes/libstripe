use crate::resources::billing::discounts::Discount;
use crate::resources::billing::plans::Plans;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::paymentmethods::source::{PaymentSourceParam, PaymentSource};
use crate::util::{List, Expandable, RangeQuery};
use crate::Client;
use std::collections::HashMap;
use crate::resources::core::customer::Customer;
use crate::resources::paymentmethods::paymentmethods::PaymentMethods;
use crate::resources::billing::invoices::Invoice;
use crate::resources::billing::subscription_schedules::SubscriptionSchedules;
use crate::resources::billing::tax_rates::TaxRate;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Subscription {
    pub id: String,
    pub object: Object,
    pub application_fee_percent: Option<i32>,
    pub billing: SubscriptionBilling,
    pub billing_cycle_anchor: i64,
    pub billing_thresholds: Option<BillingThresholds>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub collection_method: Option<String>, //TODO: Wait for more information
    pub cancel_at: Option<i64>,
    pub created: i64,
    pub current_period_end: i64,
    pub current_period_start: i64,
    pub customer: Expandable<Customer>,
    pub days_until_due: Option<i64>,
    pub default_payment_method: Option<Expandable<PaymentMethods>>,
    pub default_source: Option<Expandable<PaymentSource>>,
    pub default_tax_rates: Vec<Expandable<TaxRate>>,
    pub discount: Option<Discount>,
    pub ended_at: Option<i64>,
    pub items: List<SubscriptionItems>,
    pub livemode: bool,
    pub latest_invoice: Expandable<Invoice>,
    pub metadata: HashMap<String, String>,
    pub plan: Plans,
    pub quantity: i64,
    pub schedule: Option<Expandable<SubscriptionSchedules>>,
    pub start: i64,
    pub status: SubscriptionStatus,
    pub tax_percent: Option<f64>,
    pub trial_end: Option<i64>,
    pub trial_start: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBilling {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct BillingThresholds {
    pub amount_gte: i64,
    pub reset_billing_cycle_anchor: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Canceled,
    Unpaid,
    Incomplete,
    IncompleteExpired,
}

#[derive(Default, Serialize, Debug, PartialEq)]
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
    pub quantity: Option<i64>,
}

impl SubscriptionItems {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionItems, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::SubscriptionItems, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::SubscriptionItems, vec![id], param)
    }

    pub fn delete<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::SubscriptionItems, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::SubscriptionItems, vec![], param)
    }
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct SubscriptionParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdate_start_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<BillingThresholds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ItemParam<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    //Deprecated?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_period_days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}


#[derive(Default, Serialize, Debug, PartialEq)]
pub struct SubscriptionListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<SubscriptionBilling>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ItemParam<'a> {
    pub plan: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl Subscription {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Subscriptions, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Subscriptions, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Subscriptions, vec![id], param)
    }

    pub fn cancel<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.delete(UrlPath::Subscriptions, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Subscriptions, vec![], param)
    }
}
