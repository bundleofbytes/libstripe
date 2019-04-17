use std::fmt;
//TODO: Change so that it is only dependent on "UrlPath"
//      or move "UrlPath" to be in StripeService and have
//      path to be defined per struct
#[derive(Clone)]
pub enum UrlPath {
    Accounts,
    ApplicationFees,
    Authorizations,
    Balance,
    CardHolders,
    Charges,
    CountrySpecs,
    Coupons,
    Customers,
    Disputes,
    Events,
    File,
    FileLink,
    InvoiceItems,
    Invoices,
    IssuingDispute,
    IssuingCard,
    Order,
    PaymentIntents,
    PaymentMethods,
    Payouts,
    Plans,
    Products,
    Refunds,
    OrderReturns,
    Sigma,
    Sku,
    Sources,
    Subscriptions,
    SubscriptionItems,
    Tokens,
    Topups,
    Transactions,
    Transfers,
    WebhookEndpoints,
    Other(String),
}

impl fmt::Display for UrlPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uri = match *self {
            UrlPath::Accounts => "/accounts",
            UrlPath::Authorizations => "/issuing/authorzations",
            UrlPath::ApplicationFees => "/application_fees",
            UrlPath::Balance => "/balance",
            UrlPath::CardHolders => "/issuing/cardholders",
            UrlPath::Charges => "/charges",
            UrlPath::CountrySpecs => "/country_specs",
            UrlPath::Coupons => "/coupons",
            UrlPath::Customers => "/customers",
            UrlPath::Disputes => "/disputes",
            UrlPath::Events => "/events",
            UrlPath::File => "/files",
            UrlPath::FileLink => "/file_link",
            UrlPath::IssuingCard => "/issuing/cards",
            UrlPath::IssuingDispute => "/issuing/disputes",
            UrlPath::InvoiceItems => "/invoiceitems",
            UrlPath::Invoices => "/invoices",
            UrlPath::Order => "/order",
            UrlPath::PaymentMethods => "/payment_methods",
            UrlPath::PaymentIntents => "/payment_intents",
            UrlPath::Payouts => "/payouts",
            UrlPath::Plans => "/plans",
            UrlPath::Products => "/products",
            UrlPath::Refunds => "/refunds",
            UrlPath::OrderReturns => "/order_returns",
            UrlPath::Sigma => "/sigma",
            UrlPath::Sku => "/skus",
            UrlPath::Sources => "/sources",
            UrlPath::Subscriptions => "/subscriptions",
            UrlPath::SubscriptionItems => "/subscription_items",
            UrlPath::Tokens => "/tokens",
            UrlPath::Topups => "/topups",
            UrlPath::Transactions => "/issuing/transactions",
            UrlPath::Transfers => "/transfers",
            UrlPath::WebhookEndpoints => "/webhook_endpoints",
            UrlPath::Other(ref path) => &path,
        };
        write!(f, "{}", uri)
    }
}

#[derive(Default, Clone)]
pub struct StripePath {
    list: Vec<String>,
    query: Vec<String>,
}

impl StripePath {

    pub fn param<T: fmt::Display>(&mut self, arg: T) -> &mut Self {
        self.list.push(format!("{}", arg));
        self
    }

//    pub fn query<T: fmt::Display>(&mut self, key: &str, val: T) -> &mut Self {
//        self.query.push(format!("{}={}", key, val));
//        self
//    }

}

impl fmt::Display for StripePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = String::new();
        let list = &self.list;
        list.into_iter().for_each(|data| uri.push_str(&format!("/{}", data)) );
//        if self.query.len() != 0 {
//            let query = &self.query;
//            uri.push_str("?");
//            query.into_iter().for_each(|data| uri.push_str(&format!("{}&", data)) );
//        }
        write!(f, "{}", uri)
    }
}

//impl From<StripePath> for String {
//    fn from(p: StripePath) -> String {
//        p.to_string()
//    }
//}