
use crate::resources::common::path::UrlPath;
use crate::resources::paymentmethods::bank::BankAccount;
use crate::resources::paymentmethods::cards::Card;
use crate::util::{Deleted, List};
use crate::{Client};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExternalAccounts {
    Card(Card),
    BankAccount(BankAccount),
}

impl ExternalAccounts {
    pub fn create<B: serde::Serialize>(
        client: &Client,
        acct: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(UrlPath::Accounts, vec![acct, "external_accounts"], param)
    }

    pub fn retrieve(client: &Client, acct: &str, id: &str) -> crate::Result<Self> {
        client.get(
            UrlPath::Accounts,
            vec![acct, "external_accounts", id],
            serde_json::Map::new(),
        )
    }

    pub fn update<B: serde::Serialize>(
        client: &Client,
        acct: &str,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.post(
            UrlPath::Accounts,
            vec![acct, "external_accounts", id],
            param,
        )
    }

    pub fn delete(client: &Client, acct: &str, id: &str) -> crate::Result<Deleted> {
        client.delete(
            UrlPath::Accounts,
            vec![acct, "external_accounts", id],
            serde_json::Map::new(),
        )
    }

    pub fn list<B: serde::Serialize>(
        client: &Client,
        acct: &str,
        id: &str,
        param: B,
    ) -> crate::Result<List<Self>> {
        client.get(
            UrlPath::Accounts,
            vec![acct, "external_accounts", id],
            param,
        )
    }
}
