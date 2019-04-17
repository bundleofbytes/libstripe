use crate::resources::common::object::Object;
use crate::util::{List, RangeQuery};
use crate::resources::core::charges::Charge;
use crate::resources::common::currency::Currency;
use std::collections::HashMap;
use crate::resources::issuing::cards::IssuingShipping;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct PaymentIntent {
    pub id: String,
    pub object: Object,
    pub allowed_source_types: Vec<String>,
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
    pub customer: Option<String>,
    pub description: String,
    pub last_payment_error: Option<LastPaymentError>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub next_source_action: Option<NextSourceAction>,
    pub on_behalf_of: String,
    pub receipt_email: String,
    pub review: String,
    pub shipping: IssuingShipping,
    pub source: String,
    pub statement_descriptor: String,
    pub status: PaymentIntentsStatus,
    pub transfer_data: TransferData,
    pub transfer_group: String,
}

#[derive(Deserialize, Debug)]
pub struct NextSourceAction {
    pub authorize_with_url: Option<AuthorizeWithUrl>,
    #[serde(rename="type")]
    pub action_type: ActionType,
    pub use_stripe_sdk: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum ActionType {
    AuthorizeWithUrl,
    UseStripeSdk
}

#[derive(Deserialize, Debug)]
pub struct AuthorizeWithUrl {
    pub return_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum ConfirmationMethod {
    Secret,
    Publishable
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum CancellationReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum CaptureMethod {
    Automatic,
    Manual
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum PaymentIntentsStatus {
    RequiresSource,
    RequiresConfirmation,
    RequiresSourceAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferData {
    pub destination: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum LastPaymentErrorType {
    ApiConnectionError,
    ApiError,
    AuthenticationError,
    CardError,
    IdempotencyError,
    InvalidRequestError,
    RateLimitError
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentIntentParam<'a>{
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
}

#[derive(Default, Serialize, Debug)]
pub struct PaymentIntentListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl StripeService for PaymentIntent {}
impl<'a> StripeService for PaymentIntentParam<'a> {}
impl<'a> StripeService for PaymentIntentListParams<'a> {}

impl PaymentIntent {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, &StripePath::default(), param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::PaymentIntents, &StripePath::default().param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, &StripePath::default().param(id), param)
    }

    pub fn confirm<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, &StripePath::default().param(id).param("confirm"), param)
    }

    pub fn capture<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, &StripePath::default().param(id).param("capture"), param)
    }

    pub fn cancel<B: serde::Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::PaymentIntents, &StripePath::default().param(id).param("cancel"), param)
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::PaymentIntents, &StripePath::default(), param)
    }
}