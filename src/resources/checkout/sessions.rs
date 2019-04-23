use crate::resources::common::object::Object;
use crate::util::Expandable;
use crate::resources::core::customer::Customer;
use crate::resources::common::currency::Currency;
use crate::resources::core::paymentintents::{PaymentIntent, PaymentIntentParam};
use crate::resources::paymentmethods::paymentmethods::PaymentMethodsType;
use crate::resources::billing::subscriptions::{Subscription, SubscriptionParam};
use crate::Client;
use crate::resources::common::path::UrlPath;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Sessions {
    pub id: String,
    pub object: Object,
    pub billing_address_collection: Option<BillingAddressCollection>,
    pub cancel_url: Option<String>,
    pub client_reference_id: Option<String>,
    pub customer: Option<Expandable<Customer>>,
    pub customer_email: Option<String>,
    pub display_items: Option<Vec<DisplayItems>>,
    pub livemode: bool,
    pub locale: Option<String>,
    pub payment_intent: Expandable<PaymentIntent>,
    pub payment_method_types: Vec<PaymentMethodsType>,
    pub subscription: Option<Expandable<Subscription>>,
    pub success_url: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum BillingAddressCollection {
    Auto,
    Required
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DisplayItems {
    pub amount: i64,
    pub currency: Currency,
    pub custom: CustomDisplay,
    pub quantity: i64,
    #[serde(rename="type")]
    pub display_items_type: DisplayItemType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CustomDisplay {
    pub description: Option<String>,
    pub images: Option<String>,
    pub name: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum DisplayItemType {
    Custom,
    Plan,
    Sku
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct LineItemsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a str>
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub struct SessionsParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<BillingAddressCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<LineItemsParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<PaymentIntentParam<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<SubscriptionParam<'a>>
}

impl Sessions {

    pub fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.post(UrlPath::CheckoutSessions, vec![], param)
    }

    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::CheckoutSessions, vec![id], serde_json::Map::new())
    }

}