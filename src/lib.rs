#[macro_use]
extern crate serde_derive;

use crate::error::{Error, ErrorCode};

mod client;
pub mod error;
pub mod prelude;
pub mod resources;
pub mod util;

pub use crate::client::Client;

pub type Result<T> = ::std::result::Result<T, Error>;