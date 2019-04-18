use crate::error::{self, Error};
use reqwest::header::{self, HeaderMap};
use reqwest::multipart::Form;
use reqwest::{Client as HttpClient, Method};
use serde;
use std::str;

use crate::resources::common::path::{UrlPath};
use crate::{Result};

#[derive(Clone)]
pub struct Client {
    header: HeaderMap,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            header: HeaderMap::new(),
        }
    }
}

impl Client {
    pub fn new<S: AsRef<str>>(api_key: S) -> Client {
        let mut client = Client::default();
        client.stripe_key(api_key).unwrap_or(());
        client
    }

    pub fn stripe_key<S: AsRef<str>>(&mut self, key: S) -> Result<()> {
        self.header
            .insert(
                header::AUTHORIZATION,
                format!("Bearer {}", key.as_ref()).parse()?,
            )
            .ok_or(Error::Unknown)
            .map(|_| ())
    }

    pub fn stripe_account(&mut self, acct: &str) -> Result<()> {
        self.header
            .insert("Stripe-Account", acct.parse()?)
            .ok_or(Error::Unknown)
            .map(|_| ())
    }

    pub fn idempotent(&mut self, key: &str) -> Result<()> {
        self.header
            .insert("Idempotency-Key", key.parse()?)
            .ok_or(Error::Unknown)
            .map(|_| ())
    }

    fn headers(&self) -> HeaderMap {
        self.header.clone()
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
        let req = client
            .request(method, &uri)
            .headers(self.headers())
            .body(query);

        let mut res = match form {
            Some(multipart) => req.multipart(multipart).send()?,
            None => req.send()?,
        };

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
    }
}
