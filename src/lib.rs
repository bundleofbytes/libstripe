
#[macro_use] extern crate serde_derive;

use crate::error::{Error, ErrorCode};
use crate::resources::common::path::{UrlPath, StripePath};
use serde_json::{Value, Map};

mod client;
pub mod util;
pub mod error;
pub mod resources;
pub mod prelude;

pub use crate::client::Client;

pub type Result<T> = ::std::result::Result<T, Error>;

pub trait StripeService {

    fn uri(&self, path: UrlPath, param: &StripePath) -> String {
        format!("https://api.stripe.com/v1{}{}", path, param)
    }

    fn object() -> Value {
        Value::Object(Map::new())
    }

    fn empty(&self) -> Value {
        Self::object()
    }

}

impl StripeService for Value {}