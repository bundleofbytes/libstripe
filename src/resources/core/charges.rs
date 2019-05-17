use crate::resources::common::address::Address;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;
use crate::resources::core::refunds::Refund;
use crate::resources::paymentmethods::source::{PaymentSource, PaymentSourceParam};
use crate::util::{List, RangeQuery, Expandable};
use crate::{Client, ErrorCode};
use std::collections::HashMap;

use crate::resources::common::path::UrlPath;
use crate::resources::paymentmethods::paymentmethods::PaymentMethodsDetails;
use crate::resources::core::balance::BalanceTransaction;
use crate::resources::connect::applicationfees::ApplicationFees;
use crate::resources::core::customer::Customer;
use crate::resources::core::disputes::Dispute;
use crate::resources::billing::invoices::Invoice;
use crate::resources::orders::order::Order;
use crate::resources::fraud::review::Reviews;
use crate::resources::connect::transfers::Transfer;
use crate::resources::core::paymentintents::TransferData;
use crate::resources::connect::account::Account;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Charge {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: Option<String>,
    pub application_fee: Option<Expandable<ApplicationFees>>,
    pub application_fee_amount: Option<i32>,
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,
    pub billing_details: BillingDetails,
    pub captured: bool,
    pub created: i64,
    pub currency: Currency,
    pub customer: Option<Expandable<Customer>>,
    pub description: Option<String>,
    pub destination: Option<String>,
    pub dispute: Option<Expandable<Dispute>>,
    pub failure_code: Option<ErrorCode>,
    pub failure_message: Option<String>,
    pub fraud_details: FraudDetails,
    pub invoice: Option<Expandable<Invoice>>,
    pub livemode: bool,
    pub metadata: HashMap<String, String>,
    pub on_behalf_of: Option<Expandable<Account>>,
    pub order: Option<Expandable<Order>>,
    pub outcome: Option<Outcome>,
    pub paid: bool,
    pub payment_intent: Option<String>,
    pub payment_method: Option<String>,
    pub payment_method_details: PaymentMethodsDetails,
    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub receipt_url: String,
    pub refunded: bool,
    pub refunds: List<Refund>,
    pub review: Option<Expandable<Reviews>>,
    pub shipping: Option<ShippingDetails>,
    pub source: Option<PaymentSource>,
    pub source_transfer: Option<Expandable<Transfer>>,
    pub statement_descriptor: Option<String>,
    pub status: ChargeStatus,
    pub transfer: Option<Expandable<Transfer>>,
    pub transfer_data: Option<TransferData>,
    pub transfer_group: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BillingDetails {
    pub address: Option<Address>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PreferredLanguage {
    EN,
    DE,
    FR,
    NL,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChargeStatus {
    Succeeded,
    Pending,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Outcome {
    #[serde(rename = "type")]
    pub outcome_type: OutcomeType,
    pub network_status: NetworkStatus,
    pub reason: Option<OutcomeReason>,
    pub risk_level: RiskLevel,
    pub risk_score: i64,
    pub seller_message: Option<String>,
    pub rule: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FraudDetails {
    pub user_report: Option<UserReport>,
    pub stripe_report: Option<StripeReport>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserReport {
    Safe,
    Fraudulent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StripeReport {
    Fraudulent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ShippingDetails {
    pub name: String,
    pub address: Address,
    pub carrier: Option<String>,
    pub phone: Option<String>,
    pub tracking_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
    Authorized,
    ManualReview,
    IssuerDeclined,
    Blocked,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkStatus {
    ApprovedByNetwork,
    DeclinedByNetwork,
    NotSentToNetwork,
    ReversedAfterApproval,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Normal,
    Elevated,
    Highest,
    NotAssessed,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutcomeReason {
    ApprovedWithID,
    CallIssuer,
    CardNotSupported,
    CardVelocityExceeded,
    CurrencyNotSupported,
    DoNotHonor,
    DoNotTryAgain,
    DuplicateTransaction,
    ExpiredCard,
    Fraudulent,
    GenericDecline,
    IncorrectNumber,
    IncorrectCVC,
    IncorrectPIN,
    IncorrectZip,
    InsufficientFunds,
    InvalidAccount,
    InvalidAmount,
    InvalidCVC,
    InvalidExpiryYear,
    InvalidNumber,
    InvalidPin,
    IssuerNotAvailable,
    LostCard,
    NewAccountInformationAvailable,
    NoActionTaken,
    NotPermitted,
    PickupCard,
    PinTryExceeded,
    ProcessingError,
    ReenterTransaction,
    RestrictedCard,
    RevocationOfAllAuthorization,
    RevocationOfAuthorization,
    SecurityViolation,
    ServiceNotAllowed,
    StolenCard,
    StopPaymentOrder,
    TestmodeDeclined,
    TransactionNotAllowed,
    TryAgainLater,
    WithrawalCountLimitExceeded,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ChargeParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<&'a str>, //TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<PaymentSourceParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
    //Used for updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<FraudDetails>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct ChargeListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ChargeSourceListParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChargeSourceListParam {
    pub object: ChargeSourceObject,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChargeSourceObject {
    All,
    AlipayAccount,
    BankAccount,
    Card,
}

impl Charge {
    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Charges, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Charges, vec![id], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Charges, vec![id], param)
    }

    pub fn capture<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Charges, vec![id], param)
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Charges, vec![], param)
    }
}
