use crate::resources::paymentmethods::cards::Card;
use crate::resources::paymentmethods::bank::BankAccount;
use crate::{StripeService, Client};
use crate::resources::common::path::UrlPath;
use crate::util::{Deleted, List};
use crate::resources::common::path::StripePath;

#[derive(Deserialize, Debug)]
#[serde(rename_all="snake_case")]
pub enum ExternalAccounts {
    Card(Card),
    BankAccount(BankAccount)
}

impl StripeService for ExternalAccounts {}

impl ExternalAccounts {

    pub fn create<B: serde::Serialize + StripeService>(client: &Client, acct: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, &StripePath::default().param(acct).param("external_accounts"), param)
    }

    pub fn retrieve(client: &Client, acct: &str, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Accounts, &StripePath::default().param(acct).param("external_accounts").param(id), Self::object())
    }

    pub fn update<B: serde::Serialize + StripeService>(client: &Client, acct: &str, id: &str, param: B) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, &StripePath::default().param(acct).param("external_accounts").param(id), param)
    }

    pub fn delete(client: &Client, acct: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(UrlPath::Accounts, &StripePath::default().param(acct).param("external_accounts").param(id), Self::object())
    }

    pub fn list<B: serde::Serialize + StripeService>(client: &Client, acct: &str, id: &str, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Accounts, &StripePath::default().param(acct).param("external_accounts").param(id), param)
    }
}