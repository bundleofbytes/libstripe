use crate::resources::common::currency::Currency;
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::util::List;
use crate::{Client};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CountrySpecs {
    pub id: String,
    pub object: Object,
    pub default_currency: Currency,
    pub supported_bank_account_currencies: HashMap<Currency, Vec<String>>,
    pub supported_payment_currencies: Vec<Currency>,
    pub supported_payment_methods: Vec<PaymentMethods>,
    pub verification_fields: Option<Verification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Verification {
    pub individual: Option<EntityVerification>,
    pub company: Option<EntityVerification>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityVerification {
    pub minimum: Option<Vec<MinimumVerification>>,
    pub additional: Option<Vec<AdditionalVerification>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MinimumVerification {
    ExternalAccount,
    BusinessType,
    #[serde(rename = "company.address.city")]
    CompanyAddressCity,
    #[serde(rename = "company.address.line1")]
    CompanyAddressLine1,
    #[serde(rename = "company.address.postal_code")]
    CompanyAddressPostalCode,
    #[serde(rename = "company.address.state")]
    CompanyAddressState,
    #[serde(rename = "company.name")]
    CompanyName,
    #[serde(rename = "company.tax_id")]
    CompanyTaxID,
    #[serde(rename = "company.phone")]
    CompanyPhone,
    #[serde(rename = "business_profile.mcc")]
    BusinessProfileMCC,
    #[serde(rename = "business_profile.url")]
    BusinessProfileUrl,
    #[serde(rename = "individual.dob.day")]
    IndividualDoBDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDoBMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDoBYear,
    #[serde(rename = "individual.first_name")]
    IndividualFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualLastName,
    #[serde(rename = "individual.ssn_last_4")]
    IndividualSSNLast4,
    #[serde(rename = "individual.address.city")]
    IndividualAddressCity,
    #[serde(rename = "individual.address.line1")]
    IndividualAddressLine1,
    #[serde(rename = "individual.address.postal_code")]
    IndividualAddressPostalCode,
    #[serde(rename = "individual.address.state")]
    IndividualAddressState,
    #[serde(rename = "individual.email")]
    IndividualEmail,
    #[serde(rename = "individual.phone")]
    IndividualPhone,
    #[serde(rename = "relationship.owner")]
    RelationshipOwner,
    #[serde(rename = "relationship.account_opener")]
    RelationshipAccountOpener,
    #[serde(rename = "tos_acceptance.date")]
    TOSAcceptanceDate,
    #[serde(rename = "tos_acceptance.ip")]
    TOSAcceptanceIP,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AdditionalVerification {
    #[serde(rename = "individual.id_number")]
    IndividualIDNumber,
    #[serde(rename = "individual.verification.document")]
    IndividualVerificationDocument,
    #[serde(rename = "relationship.account_opener")]
    RelationshipAccountOpener,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PaymentMethods {
    Alipay,
    Card,
    Stripe,
    Ach,
}

#[derive(Default, Serialize, Debug)]
pub struct CountrySpecListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}

impl CountrySpecs {
    pub fn retrieve(client: &Client, country: &str) -> crate::Result<Self> {
        client.get(UrlPath::CountrySpecs, vec![country], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::CountrySpecs, vec![], param)
    }
}
