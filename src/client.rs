use crate::error::{self, Error};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client as HttpClient, Method};
use serde;
use std::str;

use crate::resources::common::path::{UrlPath};
use crate::{Result};

#[derive(Clone)]
pub struct Client {
    key: String,
    account: Option<String>,
    idempotency: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            key: String::new(),
            account: None,
            idempotency: None,
        }
    }
}

impl Client {
    pub fn new<S: AsRef<str>>(api_key: S) -> Client {
        let mut client = Client::default();
        client.stripe_key(api_key);
        client
    }

    pub fn stripe_key<S: AsRef<str>>(&mut self, key: S) {
        self.key = key.as_ref().into();
    }

    pub fn stripe_account(&mut self, acct: &str) {
        self.account = Some(acct.to_string());
    }

    pub fn idempotency(&mut self, key: &str) {
        self.idempotency = Some(key.to_string());
    }

    pub fn get<A, B>(&self, path: UrlPath, param: Vec<&str>, data: B) -> Result<A>
    where
        A: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.request(Method::GET, path, param, data, None)
    }

    pub fn post<A, B>(&self, path: UrlPath, param: Vec<&str>, data: B) -> Result<A>
    where
        A: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.request(Method::POST, path, param, data, None)
    }

    pub fn delete<A, B>(&self, path: UrlPath, param: Vec<&str>, data: B) -> Result<A>
    where
        A: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.request(Method::DELETE, path, param, data, None)
    }

    pub fn upload<A, B>(&self, path: UrlPath, param: Vec<&str>, data: B, form: Form) -> Result<A>
    where
        A: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        self.request(Method::POST, path, param, data, Some(form))
    }

    pub fn request<A, B>(
        &self,
        method: Method,
        path: UrlPath,
        param: Vec<&str>,
        data: B,
        form: Option<Form>,
    ) -> Result<A>
    where
        A: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let mut param = param
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("/");

        if param.len() > 0 {
            param = format!("/{}", param);
        }

        let uri = match path {
            UrlPath::File(true) => format!("https://files.stripe.com/v1{}{}", path, param),
            _ => format!("https://api.stripe.com/v1{}{}", path, param),
        };

        let client = HttpClient::new();
        let query = serde_qs::to_string(&data)?;
        let mut req = client
            .request(method, &uri)
            .body(query)
            .header(AUTHORIZATION, format!("Bearer {}", self.key))
            .header(USER_AGENT, "libstripe-rs/(crates.io/crates/libstripe)");

        if let Some(account) = self.account.clone() {
            req = req.header("Stripe-Account", account);
        }

        if let Some(idemp) = self.idempotency.clone() {
            req = req.header("Idempotency-Key", idemp);
        }

        if let Some(multipart) = form {
            req = req.multipart(multipart);
        }

        req.send().map_err(Error::from).and_then(|mut res| {
            if res.status().is_success() {
                res.json().map_err(Error::from)
            } else {
                let err: error::StripeErrorObject =
                    res.json().map_err(|e| error::StripeErrorObject {
                        error: error::StripeRequestObject {
                            error_type: error::ErrorType::Unknown,
                            message: Some(format!("{}", e)),
                            ..Default::default()
                        },
                    })?;
                Err(Error::from(err))
            }
        })
    }
}
