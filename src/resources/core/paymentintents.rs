use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::charges::Charge;
use crate::resources::issuing::cards::IssuingShipping;
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client};
use std::collections::HashMap;
use crate::resources::core::customer::Customer;
use crate::resources::billing::invoices::Invoice;
use crate::resources::paymentmethods::paymentmethods::{PaymentMethods, PaymentMethodsType};
use crate::resources::fraud::review::Reviews;
use crate::resources::paymentmethods::source::Source;
use crate::resources::connect::account::Account;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PaymentIntent {
    pub id: String,
    pub object: Object,
    pub amount: i32,
    pub amount_capturable: i32,
    pub amount_received: i32,
    pub application: Option<String>,
    pub application_fee_amount: Option<i64>,
    pub canceled_at: Option<i64>,
    pub cancellation_reason: Option<CancellationReason>,
    pub capture_method: CaptureMethod,
    pub charges: List<Charge>,
    pub client_secret: Option<String>,
    pub confirmation_method: ConfirmationMethod,
    pub created: i64,
    pub currency: Currency,
    pub customer: Option<Expandable<Customer>>,
    pub description: Option<String>,
    pub invoice: Option<Expandable<Invoice>>,
    pub last_payment_error: Option<LastPaymentError>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_source: Option<NextSourceAction>,
    pub on_behalf_of: Option<String>,
    pub payment_method: Option<PaymentMethods>,
    pub payment_method_type: Option<PaymentMethodsType>,
    pub receipt_email: Option<String>,
    pub review: Option<Expandable<Reviews>>,
    pub shipping: Option<IssuingShipping>,
    pub source: Option<Expandable<Source>>,
    pub statement_descriptor: Option<String>,
    pub status: PaymentIntentsStatus,
    pub transfer_data: Option<TransferData>,
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NextSourceAction {
    pub authorize_with_url: Option<AuthorizeWithUrl>,
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub use_stripe_sdk: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    AuthorizeWithUrl,
    UseStripeSdk,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AuthorizeWithUrl {
    pub return_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationMethod {
    Automatic,
    Secret,
    Publishable,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CancellationReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentsStatus {
    RequiresPaymentMethod,
    RequiresSource,
    RequiresConfirmation,
    RequiresSourceAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TransferData {
    pub destination: Expandable<Account>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LastPaymentError {
    pub error_type: LastPaymentErrorType,
    pub charge: Option<String>,
    pub code: Option<String>,
    pub decline_code: Option<String>,
    pub doc_url: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LastPaymentErrorType {
    ApiConnectionError,
    ApiError,
    AuthenticationError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
    RateLimitError,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct PaymentIntentParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_source_type: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CaptureMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_to_capture: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<CancellationReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_source_to_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<IssuingShipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<TransferData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct PaymentIntentListParams<'a> {
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

impl PaymentIntent {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::PaymentIntents, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, vec![id], param)
    }

    pub fn confirm<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, vec![id, "confirm"], param)
    }

    pub fn capture<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, vec![id, "capture"], param)
    }

    pub fn cancel<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, vec![id, "cancel"], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::PaymentIntents, vec![], param)
    }
}
