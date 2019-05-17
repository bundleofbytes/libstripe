//TODO: Impl fileupload for disputes for submitting evidence via API

use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::balance::BalanceTransaction;
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::charges::Charge;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
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
    pub uncategorized_text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EvidenceDetails {
    pub due_by: i64,
    pub has_evidence: bool,
    pub past_due: bool,
    pub submission_count: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Dispute {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub balance_transaction: String,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: Expandable<Charge>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum DisputeReason {
    CheckReturned,
    Duplicate,
    Fraudulent,
    SubscriptionCanceled,
    ProductUnacceptable,
    ProductNotReceived,
    Unrecognized,
    CreditNotProcessed,
    General,
    IncorrectAccountDetails,
    InsufficientFunds,
    BankCannotProcess,
    DebitNotAuthorized,
    CustomerInitiated,
    Other
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all="snake_case")]
pub enum DisputeStatus {
    WarningNeedsResponse,
    WarningUnderReview,
    WarningClosed,
    NeedsResponse,
    UnderReview,
    ChargeRefunded,
    Unsubmitted,
    Won,
    Lost,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct DisputeParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Evidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct DisputeListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Dispute {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Disputes, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Disputes, vec![id], param)
    }

    pub fn close(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(UrlPath::Disputes, vec![id, "close"], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Disputes, vec![], param)
    }
}
