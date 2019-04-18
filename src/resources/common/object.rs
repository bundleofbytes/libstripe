#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Object {
    #[serde(rename = "issuing.authorization")]
    Authorization,
    Balance,
    BalanceTransaction,
    Charge,
    Customer,
    Dispute,
    Event,
    File,
    List,
    FileLink,
    Payout,
    Product,
    Refund,
    Token,
    BankAccount,
    Card,
    #[serde(rename = "issuing.cardholder")]
    CardHolders,
    Source,
    Coupon,
    Discount,
    Invoice,
    #[serde(rename = "invoiceitem")]
    InvoiceItem,
    #[serde(rename = "issuing.card")]
    IssuingCard,
    #[serde(rename = "issuing.dispute")]
    IssuingDispute,
    #[serde(rename = "issuing.transaction")]
    Transactions,
    LineItem,
    Plan,
    PaymentIntent,
    Subscription,
    SubscriptionItem,
    SubscriptionSchedules,
    SubscriptionSchedulesRevisions,
    UsageRecord,
    Account,
    LoginLink,
    FeeRefund,
    #[serde(rename = "platformearning")]
    PlatformEarning,
    Person,
    CountrySpec,
    Transfer,
    TransferReversal,
    Topup,
    Review,
    Order,
    OrderItem,
    OrderReturn,
    SKU,
    ScheduledQueryRun,
    #[serde(other, skip_serializing)]
    Unknown,
}

//impl Default for Object {
//    fn default() -> Self {
//        Object::Unknown
//    }
//}
