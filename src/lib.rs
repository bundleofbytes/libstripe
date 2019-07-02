#![recursion_limit = "128"]

#[macro_use]
extern crate serde_derive;

use crate::error::{Error, ErrorCode};

mod client;
pub mod error;
pub mod prelude;
pub mod resources;
pub mod util;

pub use crate::client::Client;

#[cfg(feature = "async")]
use futures::future::Future;
#[cfg(not(feature = "async"))]
pub type Result<T> = ::std::result::Result<T, Error>;
#[cfg(feature = "async")]
pub type Result<T> = Box<Future<Item = T, Error = Error> + Send>;
