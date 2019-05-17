use crate::resources::billing::discounts::Discount;
use crate::resources::billing::plans::Plans;
use crate::resources::billing::subscriptions::{SubscriptionItems, Subscription};
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::paymentintents::PaymentIntent;
use crate::util::{List, Period, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::charges::Charge;
use crate::resources::core::customer::{Customer, CustomerShipping};
use crate::resources::paymentmethods::source::PaymentSource;
use crate::resources::paymentmethods::paymentmethods::PaymentMethods;
use crate::resources::common::address::Address;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Invoice {
    pub id: String,
    pub object: Object,
    pub account_country: String,
    pub account_name: String,
    pub amount_due: i64,
    pub amount_paid: i64,
    pub amount_remaining: i64,
    pub application_fee_amount: Option<i64>,
    pub attempt_count: i64,
    pub attempted: bool,
    pub auto_advance: bool,
    pub billing: InvoiceBilling,
    pub billing_reason: Option<InvoiceBillingReason>,
    pub charge: Option<Expandable<Charge>>,
    pub created: i64,
    pub currency: Currency,
    pub custom_fields: Option<CustomFields>,
    pub customer: Expandable<Customer>,
    pub customer_email: Option<String>,
    pub customer_address: Option<Address>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_shipping: Option<CustomerShipping>,
    pub default_payment_method: Option<Expandable<PaymentMethods>>,
    pub default_source: Option<Expandable<PaymentSource>>,
    pub description: Option<String>,
    pub discount: Option<Discount>,
    pub due_date: Option<i64>,
    pub ending_balance: Option<i64>,
    pub footer: Option<String>,
    pub hosted_invoice_url: Option<String>,
    pub invoice_pdf: Option<String>,
    pub lines: List<InvoiceLine>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_payment_attempt: Option<i64>,
    pub number: Option<String>,
    pub paid: bool,
    pub payment_intent: Option<Expandable<PaymentIntent>>,
    pub period_end: i64,
    pub period_start: i64,
    pub post_payment_credit_notes_amount: i64,
    pub pre_payment_credit_notes_amount: i64,
    pub receipt_number: Option<String>,
    pub starting_balance: i64,
    pub statement_descriptor: Option<String>,
    pub status: InvoiceStatus,
    pub status_transition: Option<StatusTransitions>,
    pub subscription: Option<Expandable<Subscription>>,
    pub subscription_proration_date: Option<i64>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub tax_percent: Option<i64>,
    pub threshold_reason: Option<ThresholdReason>,
    pub total: i64,
    pub webhooks_delivered_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CustomFields {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ThresholdReason {
    pub amount_gte: i64,
    pub item_reasons: ItemReasons,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemReasons {
    pub line_item_ids: Vec<String>,
    pub usage_gte: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StatusTransitions {
    pub finalized_at: Option<i64>,
    pub marked_uncollectible_at: Option<i64>,
    pub paid_at: Option<i64>,
    pub voided_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBilling {
    ChargeAutomatically,
    SendInvoice,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBillingReason {
    SubscriptionCycle,
    SubscriptionCreate,
    SubscriptionUpdate,
    Subscription,
    Manual,
    Upcoming,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InvoiceLine {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub discountable: bool,
    pub invoice_item: Option<String>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub period: Period,
    pub plan: Option<Plans>,
    pub proration: bool,
    pub quantity: Option<i64>,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,
    #[serde(rename = "type")]
    pub invoiceline_type: InvoiceLineType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InvoiceLineType {
    InvoiceItem,
    Subscription,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct InvoiceParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct InvoiceLineParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<SubscriptionItems>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_prorate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tax_percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_from_plan: Option<&'a str>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct InvoiceListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Invoice {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Invoices, vec![], param)
    }

    pub fn retrieve(client: &Client, invoice: &str) -> crate::Result<Self> {
        client.get(UrlPath::Invoices, vec![invoice], serde_json::Map::new())
    }

    pub fn retrieve_upcoming<B: serde::Serialize>(
        client: &Client,
        param: B,
    ) -> crate::Result<Self> {
        client.get(UrlPath::Invoices, vec!["upcoming"], param)
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        invoice_id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Invoices, vec![invoice_id], param)
    }

    pub fn delete(client: &Client, invoice_id: &str) -> crate::Result<Self> {
        client.delete(UrlPath::Invoices, vec![invoice_id], serde_json::Map::new())
    }

    pub fn finalize<B: serde::Serialize>(
        client: &Client,
        invoice_id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Invoices, vec![invoice_id, "finalize"], param)
    }

    pub fn pay<B: serde::Serialize>(
        client: &Client,
        invoice_id: &str,
        source: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Invoices, vec![invoice_id, "pay"], source)
    }

    pub fn send(client: &Client, invoice_id: &str) -> crate::Result<Self> {
        client.post(
            UrlPath::Invoices,
            vec![invoice_id, "send"],
            serde_json::Map::new(),
        )
    }

    pub fn void(client: &Client, invoice_id: &str) -> crate::Result<Self> {
        client.post(
            UrlPath::Invoices,
            vec![invoice_id, "void"],
            serde_json::Map::new(),
        )
    }

    pub fn uncollectible(client: &Client, invoice_id: &str) -> crate::Result<Self> {
        client.post(
            UrlPath::Invoices,
            vec![invoice_id, "mark_uncollectible"],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Invoices, vec![], param)
    }
}

impl InvoiceLine {
    pub fn retrieve_upcoming_lines<B: serde::Serialize>(
        client: &Client,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(UrlPath::Invoices, vec!["upcoming", "lines"], param)
    }

    pub fn retrieve_lines<B: serde::Serialize>(
        client: &Client,
        invoice: &str,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(UrlPath::Invoices, vec![invoice, "lines"], param)
    }
}
