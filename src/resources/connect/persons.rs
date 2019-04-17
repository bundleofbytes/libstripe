use crate::resources::common::object::Object;
use crate::resources::common::address::Address;
use std::collections::HashMap;
use crate::resources::connect::account::Requirements;
use serde::Serialize;
use crate::{StripeService, Client};
use crate::resources::common::path::{UrlPath, StripePath};
use crate::util::{Deleted, List};

#[derive(Deserialize, Debug)]
pub struct Persons {
    pub id: String,
    pub object: Object,
    pub account: String,
    pub address: Address,
    pub address_kana: Address,
    pub address_kanji: Address,
    pub created: i64,
    pub dob: DayOfBirth,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub first_name_kana: Option<String>,
    pub last_name_kana: Option<String>,
    pub first_name_kanji: Option<String>,
    pub last_name_kanji: Option<String>,
    pub gender: Gender,
    pub id_number_provided: bool,
    pub maiden_name: String,
    pub metadata: HashMap<String, String>,
    pub phone: String,
    pub relationship: Relationship,
    pub requirements: Requirements,
    pub ssn_last_4_provided: bool,
    pub verification: AccountVerification
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub account_opener: bool,
    pub director: bool,
    pub owner: bool,
    pub percent_ownership: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum Gender {
    Male,
    Female
}


#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DayOfBirth {
    pub day: Option<i16>,
    pub month: Option<i16>,
    pub year: Option<i16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountVerification {
    pub details: Option<String>,
    pub details_code: Option<DetailsCode>,
    pub status: Option<DocumentStatus>,
    pub document: Option<Document>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    pub back: Option<String>,
    pub details: Option<String>,
    pub details_code: Option<DetailsCode>,
    pub front: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="lowercase")]
pub enum DocumentStatus {
    Unverified,
    Pending,
    Verified
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum DetailsCode {
    DocumentCorrupt,
    DocumentFailedCopy,
    DocumentNotReadable,
    DocumentFailedGreyscale,
    DocumentNotUploaded,
    DocumentIdTypeNotSupported,
    DocumentIdCountryNotSupported,
    DocumentFailedOther,
    DocumentFraudulent,
    DocumentInvalid,
    DocumentManipulated,
    DocumentMissingBack,
    DocumentMissingFront,
    DocumentPhotoMismatch,
    DocumentTooLarge,
    DocumentFailedTestMode,
    ScanNameMismatch,
    FailedKeyedIdentity,
    FailedOther,
}

#[derive(Default, Debug, Serialize)]
pub struct PersonsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DayOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<AccountVerification>
}

#[derive(Default, Serialize, Debug)]
pub struct PersonsListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Relationship>,
}

impl StripeService for Persons {}
impl<'a> StripeService for PersonsParam<'a> {}
impl<'a> StripeService for PersonsListParams<'a> {}

impl Persons {

    pub fn create<B: Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, &StripePath::default().param(id).param("persons"), param)
    }

    pub fn retrieve(client: &Client, aid: &str, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Accounts, &StripePath::default().param(aid).param("persons").param(id), Self::object())
    }

    pub fn update<B: Serialize + StripeService>(client: &Client, aid: &str, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, &StripePath::default().param(aid).param("persons").param(id), param)
    }

    pub fn delete(client: &Client, aid: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Accounts, &StripePath::default().param(aid).param("persons").param(id), Self::object())
    }

    pub fn list<B: Serialize + StripeService>(client: &Client, id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Accounts, &StripePath::default().param(id).param("persons"), param)
    }

}