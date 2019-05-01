use std::fmt;

#[derive(Deserialize, Serialize, Clone)]
pub enum UrlPath {
    #[serde(rename="/accounts")]
    Accounts,
    #[serde(rename="/application_fees")]
    ApplicationFees,
    #[serde(rename="/issuing/authorizations")]
    Authorizations,
    #[serde(rename="/balance")]
    Balance,
    #[serde(rename="/issuing/cardholders")]
    CardHolders,
    #[serde(rename="/charges")]
    Charges,
    #[serde(rename="/checkout/sessions")]
    CheckoutSessions,
    #[serde(rename="/country_specs")]
    CountrySpecs,
    #[serde(rename="/coupons")]
    Coupons,
    #[serde(rename="/credit_notes")]
    CreditNote,
    #[serde(rename="/customers")]
    Customers,
    #[serde(rename="/disputes")]
    Disputes,
    #[serde(rename="/events")]
    Events,
    #[serde(rename="/files")]
    File(bool),
    #[serde(rename="/file_link")]
    FileLink,
    #[serde(rename="/invoiceitems")]
    InvoiceItems,
    #[serde(rename="/invoices")]
    Invoices,
    #[serde(rename="/issuing/disputes")]
    IssuingDispute,
    #[serde(rename="/issuing/cards")]
    IssuingCard,
    #[serde(rename="/order")]
    Order,
    #[serde(rename="/payment_intents")]
    PaymentIntents,
    #[serde(rename="/payment_methods")]
    PaymentMethods,
    #[serde(rename="/payouts")]
    Payouts,
    #[serde(rename="/plans")]
    Plans,
    #[serde(rename="/products")]
    Products,
    #[serde(rename="/refunds")]
    Refunds,
    #[serde(rename="/order_returns")]
    OrderReturns,
    #[serde(rename="/radar/value_lists")]
    RadarValueList,
    #[serde(rename="/radar/value_list_items")]
    RadarValueListItems,
    #[serde(rename="/reviews")]
    Reviews,
    #[serde(rename="/sigma/scheduled_query_runs")]
    Sigma,
    #[serde(rename="/skus")]
    Sku,
    #[serde(rename="/sources")]
    Sources,
    #[serde(rename="/subscriptions")]
    Subscriptions,
    #[serde(rename="/subscription_items")]
    SubscriptionItems,
    #[serde(rename="/subscription_schedules")]
    SubscriptionSchedules,
    #[serde(rename="/tax_rates")]
    TaxRates,
    #[serde(rename="/terminal/connection_tokens")]
    TerminalConnectionTokens,
    #[serde(rename="/terminal/locations")]
    TerminalLocations,
    #[serde(rename="/terminal/readers")]
    TerminalReaders,
    #[serde(rename="/tokens")]
    Tokens,
    #[serde(rename="/topups")]
    Topups,
    #[serde(rename="/issuing/transactions")]
    Transactions,
    #[serde(rename="/transfers")]
    Transfers,
    #[serde(rename="/webhook_endpoints")]
    WebhookEndpoints,
}

impl fmt::Display for UrlPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uri = serde_json::to_string(self)
            .unwrap_or_default()
            .replace("\"", "");
        write!(f, "{}", uri)
    }
}