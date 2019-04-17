
use serde_json;
use serde_qs;
use http;
//use crate::util::to_snakecase;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use reqwest;

macro_rules! from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}
#[derive(Debug)]
pub enum Error {
    Stripe(StripeErrorObject),
    Http(reqwest::Error),
    Json(serde_json::error::Error),
    Uri(http::uri::InvalidUri),
    Io(io::Error),
    QsError(serde_qs::Error),
    HeaderError(reqwest::header::InvalidHeaderValue),
    Unknown
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Stripe(ref err) => write!(f, ": {:?}", err),
            Error::Http(ref err) => write!(f, ": {}", err),
            Error::Json(ref err) => write!(f, ": {}", err),
            Error::Uri(ref err) => write!(f, ": {}", err),
            Error::Io(ref err) => write!(f, ": {}", err),
            Error::QsError(ref err) => write!(f, ": {}", err),
            Error::HeaderError(ref err) => write!(f, ": {}", err),
            Error::Unknown => write!(f, ": unknown error"),
        }
    }
}

from_error!(StripeErrorObject, Error::Stripe);
from_error!(reqwest::Error, Error::Http);
from_error!(reqwest::header::InvalidHeaderValue, Error::HeaderError);
from_error!(serde_json::Error, Error::Json);
from_error!(http::uri::InvalidUri, Error::Uri);
from_error!(io::Error, Error::Io);
from_error!(serde_qs::Error, Error::QsError);

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Http(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::Uri(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::HeaderError(ref err) => Some(err),
            Error::QsError(ref err) => Some(err),
            _ => None,
        }
    }
}


#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum ErrorType {
    ApiError,
    ApiConnectionError,
    AuthenticationError,
    CardError,
    InvalidRequestError,
    RateLimitError,
    ValidationError,
    #[serde(skip_deserializing)]
    Unknown,
}

impl Default for ErrorType {
    fn default() -> Self {
        ErrorType::Unknown
    }
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all="snake_case")]
pub enum ErrorCode {
    InvalidNumber,
    InvalidExpiryMonth,
    InvalidExpiryYear,
    InvalidCvc,
    InvalidSwipeData,
    IncorrectNumber,
    ExpiredCard,
    IncorrectCvc,
    IncorrectZip,
    CardDeclined,
    Missing,
    ProcessingError,
    ParameterMissing,
    AccountAlreadyExists,
    AccountCountryInvalidAddress,
    AccountInvalid,
    AccountNumberInvalid,
    AlipayUpgradeRequired,
    AmountTooLarge,
    AmountTooSmall,
    ApiKeyExpired,
    BalanceInsufficient,
    BankAccountExists,
    BankAccountUnusable,
    BankAccountUnverified,
    ChargeAlreadyCaptured,
    ChargeAlreadyRefunded,
    ChargeExpiredForCapture,
    CountryUnsupported,
    CouponExpired,
    CustomerMaxSubscriptions,
    EmailInvalid,
    InstantPayoutsUnsupported,
    InvalidCardType,
    InvalidChargeAmount,
    InvalidSourceUsage,
    InvoiceNoCustomerLineItems,
    InvoiceNoSubscriptionLineItems,
    InvoiceNotEditable,
    InvoiceUpcomingNone,
    LivemodeMismatch,
    OrderCreationFailed,
    OrderRequiredSettings,
    OrderStatusInvalid,
    OrderUpstreamTimeout,
    OutOfInventory,
    ParameterInvalidEmpty,
    ParameterInvalidInteger,
    ParameterInvalidStringBlank,
    ParameterInvalidStringEmpty,
    ParameterUnknown,
    PaymentMethodUnactivated,
    PayoutNotAllowed,
    PlatformApiKeyExpired,
    PostalCodeInvalid,
    ProductInactive,
    RateLimit,
    ResourceAlreadyExists,
    ResourceMissing,
    RoutingNumberInvalid,
    SecretKeyRequired,
    SepaUnsupportedAccount,
    ShippingCalculationFailed,
    SkuInactive,
    StateUnsupported,
    TaxIdInvalid,
    TaxesCalculationFailed,
    TestmodeChargesOnly,
    TlsVersionUnsupported,
    TokenAlreadyUsed,
    TokenInUse,
    TransfersNotAllowed,
    UrlInvalid,
    InvalidUtf8InPostBody
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct StripeErrorObject {
    pub error: StripeRequestObject,
}

#[derive(Debug, Default, Deserialize)]
pub struct StripeRequestObject {
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub charge: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    pub code: Option<ErrorCode>,
    pub decline_code: Option<String>,
    pub param: Option<String>,
    pub doc_url: Option<String>
}

impl fmt::Display for StripeErrorObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error.error_type)?;
        if let Some(ref message) = self.error.message {
            write!(f, ": {}", message)?;
        }
        Ok(())
    }
}
