use crate::resources::common::object::Object;
use crate::resources::common::path::UrlPath;
use crate::resources::core::customer::Customer;
use crate::util::{Deleted, Expandable, List};
use crate::Client;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CustomerTaxID {
    pub id: String,
    pub object: Object,
    pub country: String,
    pub created: i64,
    pub customer: Expandable<Customer>,
    pub livemode: bool,
    #[serde(rename = "type")]
    pub taxid_type: TaxIDType,
    pub value: String,
    pub verification: TaxIDVerification,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIDType {
    EuVat,
    NzGst,
    AuAbn,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TaxIDVerification {
    pub status: TaxIDVerificationStatus,
    pub verified_address: Option<String>,
    pub verified_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaxIDVerificationStatus {
    Pending,
    Unavailable,
    Unverified,
    Verified,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct CustomerTaxIDParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub taxid_type: Option<TaxIDType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

#[derive(Default, Serialize, Debug, PartialEq)]
pub struct CustomerTaxIDListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<&'a str>>,
}

impl CustomerTaxID {
    pub fn create<B: serde::Serialize>(
        client: &Client,
        customer: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Customers, vec![customer, "tax_ids"], param)
    }

    pub fn retrieve(client: &Client, customer: &str, id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::Customers,
            vec![customer, "tax_ids", id],
            serde_json::Map::new(),
        )
    }

    pub fn delete(client: &Client, customer: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(
            UrlPath::Customers,
            vec![customer, "tax_ids", id],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: serde::Serialize>(
        client: &Client,
        customer: &str,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(UrlPath::Customers, vec![customer, "tax_ids"], param)
    }
}
