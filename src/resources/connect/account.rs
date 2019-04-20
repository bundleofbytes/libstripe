use crate::resources::billing::plans::Interval;
use crate::resources::common::address::Address;
use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::connect::persons::{Persons, PersonsParam};
use crate::resources::paymentmethods::bank::{BankAccount, BankAccountParam};
use crate::util::{Deleted, List};
use crate::{Client};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub object: Object,
    pub business_profile: Option<BusinessProfile>,
    pub business_type: BusinessType,
    pub capabilities: Capabilities,
    pub charges_enabled: bool,
    pub company: Option<Company>,
    pub country: String,
    pub created: Option<i64>,
    pub default_currency: Currency,
    pub details_submitted: bool,
    pub email: String,
    pub external_accounts: List<BankAccount>,
    pub individual: Persons,
    pub metadata: HashMap<String, String>,
    pub payouts_enabled: bool,
    pub requirements: Requirements,
    pub settings: AccountSettings,
    pub tos_acceptance: TosAcceptance,
    #[serde(rename = "type")]
    pub account_type: AccountType,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Requirements {
    pub current_deadline: Option<i64>,
    pub currently_due: Option<Vec<String>>, //TODO
    pub disabled_reason: Option<DisabledReason>,
    pub eventually_due: Option<Vec<String>>,
    pub past_due: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DisabledReason {
    #[serde(rename = "requirements.past_due")]
    RequirementsPastDue,
    #[serde(rename = "requirements.pending_verification")]
    RequirementsPendingVerification,
    #[serde(rename = "rejected.fraud")]
    RejectedFraud,
    #[serde(rename = "rejected.terms_of_service")]
    RejectedTermsOfService,
    #[serde(rename = "rejected.list")]
    RejectedListed,
    #[serde(rename = "rejected.other")]
    RejectedOther,
    Listed,
    UnderReview,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BusinessType {
    Individual,
    Company,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BusinessProfile {
    pub mcc: Option<String>, //MerchantCategories,
    pub name: Option<String>,
    pub product_description: Option<String>,
    pub support_address: Option<Address>,
    pub support_email: Option<String>,
    pub support_phone: Option<String>,
    pub support_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct AccountSettings {
    pub branding: Option<Branding>,
    pub card_payments: Option<CardPayments>,
    pub dashboard: Option<Dashboard>,
    pub payments: Option<Payments>,
    pub payouts: Option<Payouts>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Branding {
    pub icon: Option<String>,
    pub logo: Option<String>,
    pub primary_color: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CardPayments {
    pub decline_on: Option<DeclineOn>,
    pub statement_descriptor_prefix: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Dashboard {
    pub display_name: String,
    pub timezone: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Payments {
    pub statement_descriptor: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Payouts {
    pub debit_negative_balance: Option<bool>,
    pub schedule: Option<PayoutsSchedule>,
    pub statement_descriptor: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct PayoutsSchedule {
    pub dalay_days: Option<i32>,
    pub interval: Option<Interval>,
    pub monthly_anchor: Option<i32>,
    pub weekly_anchor: Option<i32>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct TosAcceptance {
    pub date: Option<i64>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Capabilities {
    pub card_payments: Option<String>,
    pub legacy_payments: Option<String>,
    pub platform_payments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    STANDARD,
    EXPRESS,
    CUSTOM,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Company {
    pub address: Option<Address>,
    pub address_kana: Option<Address>,
    pub address_kanji: Option<Address>,
    pub name: Option<String>,
    pub name_kana: Option<String>,
    pub name_kanji: Option<String>,
    pub directors_provided: bool,
    pub tax_id_registrar: Option<String>,
    pub vat_id_provided: Option<String>,
    pub tax_id_provided: bool,
    pub phone: String,
    pub owners_provided: bool,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CompanyParam<'a> {
    pub address: Option<Address>,
    pub address_kana: Option<Address>,
    pub address_kanji: Option<Address>,
    pub name: Option<&'a str>,
    pub name_kana: Option<&'a str>,
    pub name_kanji: Option<&'a str>,
    pub directors_provided: Option<bool>,
    pub tax_id_registrar: Option<&'a str>,
    pub vat_id: Option<&'a str>,
    pub tax_id: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub owners_provided: Option<bool>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DeclineOn {
    pub avs_failure: bool,
    pub cvc_failure: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginLink {
    pub object: Object,
    pub created: i64,
    pub url: String,
}

#[derive(Default, Serialize, Debug)]
pub struct AccountParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<BusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_capabilities: Option<Capabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<BankAccountParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonsParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Requirements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<AccountSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<TosAcceptance>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
}

impl Account {
    pub fn create<B: Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Accounts, vec![id], serde_json::Map::new())
    }

    pub fn update<B: Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, vec![id], param)
    }

    pub fn delete(client: &Client, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Accounts, vec![id], serde_json::Map::new())
    }

    pub fn reject<B: Serialize>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, vec![id, "reject"], param)
    }

    pub fn list<B: Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Accounts, vec![], param)
    }
}

impl LoginLink {
    pub fn create(client: &Client, account: &str) -> crate::Result<Self> {
        client.post(
            UrlPath::Accounts,
            vec![account, "login_links"],
            serde_json::Map::new(),
        )
    }
}
