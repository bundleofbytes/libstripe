

use reqwest::header::{self, HeaderMap};
use reqwest::{Method, Client as HttpClient};
use reqwest::multipart::Form;
use crate::error::{self, Error};
use std::str;
use serde;

use crate::{Result, StripeService};
use crate::resources::common::path::{UrlPath, StripePath};

#[derive(Clone)]
pub struct Client {
    header: HeaderMap,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            header: HeaderMap::new()
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
        self.header.insert(header::AUTHORIZATION, format!("Bearer {}", key.as_ref()).parse()?).ok_or(Error::Unknown).map(|_| ())
    }

    pub fn stripe_account(&mut self, acct: &str) -> Result<()> {
        self.header.insert("Stripe-Account", acct.parse()?).ok_or(Error::Unknown).map(|_| ())
    }

    pub fn idempotent(&mut self, key: &str) -> Result<()> {
        self.header.insert("Idempotency-Key", key.parse()?).ok_or(Error::Unknown).map(|_| ())
    }

    fn headers(&self) -> HeaderMap {
        self.header.clone()
    }

    pub fn get<A, B>(&self, path: UrlPath, param: &StripePath, data: B) -> Result<A>
        where A: serde::de::DeserializeOwned,
              B: serde::Serialize + StripeService
    {
        self.request(Method::GET, path, param, data, None)
    }

    pub fn post<A, B>(&self, path: UrlPath, param: &StripePath, data: B) -> Result<A>
        where A: serde::de::DeserializeOwned,
              B: serde::Serialize + StripeService
    {
        self.request(Method::POST, path, param, data, None)
    }

    pub fn delete<A, B>(&self, path: UrlPath, param: &StripePath, data: B) -> Result<A>
        where A: serde::de::DeserializeOwned,
              B: serde::Serialize + StripeService
    {
        self.request(Method::DELETE, path, param, data, None)
    }

    pub fn upload<A, B>(&self, path: UrlPath, param: &StripePath, data: B, form: Form) -> Result<A>
        where A: serde::de::DeserializeOwned,
              B: serde::Serialize + StripeService
    {
        self.request(Method::POST, path, param, data, Some(form))
    }

    pub fn request<A, B>(&self, method: Method, path: UrlPath, param: &StripePath, data: B, form: Option<Form>) -> Result<A>
        where A: serde::de::DeserializeOwned,
              B: serde::Serialize + StripeService
    {
        let uri = data.uri(path, param);
        let client = HttpClient::new();
        let query = serde_qs::to_string(&data)?;
        let req = client.request(method, &uri)
                                      .headers(self.headers())
                                      .body(query);
        
        let mut res = match form {
            Some(multipart) => req.multipart(multipart).send()?,
            None => req.send()?
        };
        
        if res.status().is_success() {
            res.json().map_err(Error::from)
        } else {
            let err: error::StripeErrorObject = res.json().map_err(|e| {
                error::StripeErrorObject {
                    error: error::StripeRequestObject {
                        error_type: error::ErrorType::Unknown,
                        message: Some(format!("{}", e)),
                        ..Default::default()
                    }
                }
            })?;
            Err(Error::from(err))
        }

    }

}