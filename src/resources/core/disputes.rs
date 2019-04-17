

//TODO: Impl fileupload for disputes for submitting evidence via API

use crate::resources::common::object::Object;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::util::{RangeQuery, List};
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Evidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct EvidenceDetails {
    pub due_by: i64,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: i64
}

#[derive(Deserialize, Debug)]
pub struct Dispute {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: String,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: String,
    pub created: i64,
    pub currency: Currency,
    pub evidence: Evidence,
    pub evidence_details: EvidenceDetails,
    pub is_charge_refundable: bool,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub reason: DisputeReason,
    pub status: DisputeStatus,
}

#[derive(Deserialize, Debug)]
pub enum DisputeReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "fraudulent")]
    Fraudulent,
    #[serde(rename = "subscription_canceled")]
    SubscriptionCanceled,
    #[serde(rename = "product_unacceptable")]
    ProductUnacceptable,
    #[serde(rename = "product_not_received")]
    ProductNotReceived,
    #[serde(rename = "unrecognized")]
    Unrecognized,
    #[serde(rename = "credit_not_processed")]
    CreditNotProcessed,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "incorrect_account_details")]
    IncorrectAccountDetails,
    #[serde(rename = "insufficient_funds")]
    InsufficientFunds,
    #[serde(rename = "bank_cannot_process")]
    BankCannotProcess,
    #[serde(rename = "debit_not_authorized")]
    DebitNotAuthorized,
    #[serde(rename = "customer_initiated")]
    CustomerInitiated
}

#[derive(Deserialize, Debug)]
pub enum DisputeStatus {
    #[serde(rename = "warning_needs_response")]
    WarningNeedsResponse,
    #[serde(rename = "warning_under_review")]
    WarningUnderReview,
    #[serde(rename = "warning_closed")]
    WarningClosed,
    #[serde(rename = "needs_response")]
    NeedsResponse,
    #[serde(rename = "under_review")]
    UnderReview,
    #[serde(rename = "charge_refunded")]
    ChargeRefunded,
    #[serde(rename = "won")]
    Won,
    #[serde(rename = "lost")]
    Lost
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DisputeParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>
}

#[derive(Default, Serialize, Debug)]
pub struct DisputeListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl StripeService for Dispute {}
impl StripeService for DisputeParam {}
impl<'a> StripeService for DisputeListParam<'a> {}

impl Dispute {
    
    pub fn retrieve(client: &Client, dispute_id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Disputes, &StripePath::default().param(dispute_id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, dispute_id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Disputes, &StripePath::default().param(dispute_id), param)
    }

    pub fn close(client: &Client, dispute_id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Disputes, &StripePath::default().param(dispute_id).param("close"), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Disputes, &StripePath::default(), param)
    }

}