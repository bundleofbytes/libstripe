use crate::resources::common::object::Object;
use crate::resources::common::currency::Currency;
use crate::util::{RangeQuery, List};
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
pub struct Balance {
    pub object: Object,
    pub available: Vec<BalanceSource>,
    pub connect_reserved: Vec<BalanceSource>,
    pub pending: Vec<BalanceSource>,
    pub livemode: bool,
}

#[derive(Deserialize, Debug)]
pub struct BalanceSource {
    pub currency: Currency,
    pub amount: i64,
    pub source_types: Option<BalanceSourceType>
}

#[derive(Deserialize, Debug)]
pub struct BalanceSourceType {
    pub card: Option<i64>,
    pub bank_account: Option<i64>,
    pub bitcoin_receiver: Option<i64>
}

#[derive(Deserialize, Debug)]
pub struct BalanceTransaction {
    pub id: String,
    pub object: Object,
    pub amount: i64,
    pub available_on: i64,
    pub created: i64,
    pub currency: Currency,
    pub description: Option<String>,
    pub exchange_rate: Option<f64>,
    pub fee: i64,
    pub fee_details: Vec<FeeDetails>,
    pub net: i64,
    pub source: String,
    pub status: BalanceStatus,
    #[serde(rename="type")]
    pub balance_type: BalanceType
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum BalanceStatus {
    Available,
    Pending
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum BalanceType {
    Adjustment,
    ApplicationFee,
    ApplicationFeeRefund,
    Charge,
    Payment,
    PaymentFailureRefund,
    PaymentRefund,
    Refund,
    Transfer,
    TransferRefund,
    Payout,
    PayoutCancel,
    PayoutFailure,
    Validation,
    StripeFee,
    NetworkCost,
}

#[derive(Deserialize, Debug)]
pub struct FeeDetails {
    pub amount: i64,
    pub application: Option<String>,
    pub currency: Currency,
    pub description: String,
    #[serde(rename="type")]
    pub fee_type: FeeType
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum FeeType {
    ApplicationFee,
    StripeFee,
    Tax
}

#[derive(Default, Serialize, Debug)]
pub struct BalanceListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_on: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="type")]
    pub balance_type: Option<BalanceType>,
}

impl StripeService for Balance {}
impl StripeService for BalanceTransaction {}
impl<'a> StripeService for BalanceListParams<'a> {}

impl Balance {

    pub fn retrieve(client: &Client) -> crate::Result<Self> {
        client.get(UrlPath::Balance, &StripePath::default(), Balance::object())
    }

}

impl BalanceTransaction {

    pub fn retrieve_transaction(client: &Client, txn: String) -> crate::Result<Self> {
        client.get(UrlPath::Balance, &StripePath::default().param("history").param(txn), Balance::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Balance, &StripePath::default().param("history"), param)
    }

}