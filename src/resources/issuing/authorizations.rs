use crate::resources::common::category::MerchantCategories;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::core::balance::BalanceTransaction;
use crate::util::{List, RangeQuery, Expandable};
use crate::Client;
use std::collections::HashMap;
use crate::resources::issuing::cards::IssuingCard;
use crate::resources::issuing::transactions::Transactions;
use crate::resources::issuing::cardholders::CardHolders;

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorizations {
    pub id: String,
    pub object: Object,
    pub approved: bool,
    pub authorization_method: AuthorizationMethod,
    pub authorized_amount: i64,
    pub authorized_currency: Currency,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub card: Option<IssuingCard>,
    pub cardholder: Option<Expandable<CardHolders>>,
    pub created: i64,
    pub held_amount: i64,
    pub held_currency: Currency,
    pub is_held_amount_controllable: bool,
    pub livemode: bool,
    pub merchant_data: MerchantData,
    pub metadata: HashMap<String, String>,
    pub pending_authorized_amount: i64,
    pub pending_held_amount: i64,
    pub request_history: Vec<RequestHistory>,
    pub status: AuthorizationStatus,
    pub transactions: Option<Transactions>,
    pub verification_data: VerificationData,
    pub wallet_provider: Option<WalletProvider>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum WalletProvider {
    ApplePay,
    GooglePay,
    SamsungPay
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationMethod {
    KeyedIn,
    Swipe,
    Chip,
    Contactless,
    Online,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MerchantData {
    pub category: MerchantCategories,
    pub city: String,
    pub country: String,
    pub name: String,
    pub network_id: String,
    pub postal_code: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestHistory {
    pub approved: bool,
    pub authorized_amount: i64,
    pub authorized_currency: Currency,
    pub created: i64,
    pub held_amount: i64,
    pub held_currency: Currency,
    pub reason: RequestReason,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RequestReason {
    AuthorizationControls,
    CardActive,
    CardInactive,
    InsufficientFunds,
    AccountComplianceDisabled,
    AccountInactive,
    SuspectedFraud,
    WebhookApproved,
    WebhookDeclined,
    WebhookTimeout,
    ForcedCardAuthenticationFailed,
    ForcedCardExpired,
    ForcedCardLost,
    ForcedCardStolen,
    ForcedDoNotHonor,
    ForcedIncorrectPin,
    ForcedInsufficientFunds,
    ForcedInvalidAccountNumber,
    ForcedInvalidTransaction,
    ForcedSuspectedFraud,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationData {
    pub address_line1_check: VerificationCheck,
    pub address_zip_check: VerificationCheck,
    pub cvc_check: VerificationCheck,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VerificationCheck {
    Match,
    Mismatch,
    NotProvided,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationStatus {
    Pending,
    Reversed,
    Closed,
}

#[derive(Debug, Default, Serialize)]
pub struct AuthorizationsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<&'a str, &'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Debug, Default, Serialize)]
pub struct AuthorizationsListParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AuthorizationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl Authorizations {

    pub fn retrieve(client: &Client) -> crate::Result<Self> {
        client.get(UrlPath::Authorizations, vec![], serde_json::Map::new())
    }

    pub fn update<B: serde::Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Authorizations, vec![id], param)
    }

    pub fn approve<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Authorizations, vec![id, "approve"], param)
    }

    pub fn decline<B: serde::Serialize>(client: &Client, id: &str) -> crate::Result<Self> {
        client.post(
            UrlPath::Authorizations,
            vec![id, "decline"],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Authorizations, vec![], param)
    }

}
