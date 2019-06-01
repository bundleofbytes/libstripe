use crate::resources::billing::coupons::Coupon;
use crate::resources::billing::discounts::Discount;
use crate::resources::billing::invoiceitems::InvoiceItems;
use crate::resources::billing::invoices::Invoice;
use crate::resources::billing::plans::Plans;
use crate::resources::billing::subscriptions::{Subscription, SubscriptionItems};
use crate::resources::common::object::Object;

use crate::resources::common::path::UrlPath;
use crate::resources::connect::transfers::Transfer;
use crate::resources::core::balance::Balance;
use crate::resources::core::charges::Charge;
use crate::resources::core::customer::Customer;
use crate::resources::core::disputes::Dispute;
use crate::resources::core::payout::Payout;
use crate::resources::core::product::Products;
use crate::resources::core::refunds::Refund;
use crate::resources::core::tokens::Tokens;
use crate::resources::orders::order::{Order, OrderReturn};
use crate::resources::orders::sku::Sku;
use crate::resources::paymentmethods::bank::BankAccount;
use crate::resources::paymentmethods::cards::Card;
use crate::resources::paymentmethods::source::Source;
use crate::util::List;
use crate::{Client};
use crate::resources::billing::subscription_schedules::SubscriptionSchedules;
use crate::resources::core::file::File;
use crate::resources::issuing::authorizations::Authorizations;
use crate::resources::issuing::cards::IssuingCard;
use crate::resources::issuing::cardholders::CardHolders;
use crate::resources::issuing::dispute::IssuingDispute;
use crate::resources::issuing::transactions::Transactions;
use serde_json::Value;
use crate::resources::core::paymentintents::PaymentIntent;
use crate::resources::paymentmethods::paymentmethods::PaymentMethods;
use crate::resources::connect::persons::Persons;
use crate::resources::fraud::review::Reviews;
use crate::resources::connect::topup::Topup;
use crate::resources::connect::applicationfees::ApplicationFees;
use crate::resources::connect::applicationrefunds::ApplicationFeeRefunds;
use crate::resources::checkout::sessions::Sessions;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EventType {
    #[serde(rename = "account.updated")]
    AccountUpdated,
    #[serde(rename = "account.application.authorized")]
    AccountApplicationAuthorized,
    #[serde(rename = "account.application.deauthorized")]
    AccountApplicationDeauthorized,
    #[serde(rename = "account.external_account.created")]
    AccountExternalAccountCreated,
    #[serde(rename = "account.external_account.deleted")]
    AccountExternalAccountDeleted,
    #[serde(rename = "account.external_account.updated")]
    AccountExternalAccountUpdated,
    #[serde(rename = "application_fee.created")]
    ApplicationFeeCreated,
    #[serde(rename = "application_fee.refunded")]
    ApplicationFeeRefunded,
    #[serde(rename = "application_fee.refund.updated")]
    ApplicationFeeRefundUpdated,
    #[serde(rename = "balance.available")]
    BalanceAvailable,
    #[serde(rename = "charge.captured")]
    ChargeCaptured,
    #[serde(rename = "charge.expired")]
    ChargeExpired,
    #[serde(rename = "charge.failed")]
    ChargeFailed,
    #[serde(rename = "charge.pending")]
    ChargePending,
    #[serde(rename = "charge.refunded")]
    ChargeRefunded,
    #[serde(rename = "charge.succeeded")]
    ChargeSucceeded,
    #[serde(rename = "charge.updated")]
    ChargeUpdated,
    #[serde(rename = "charge.dispute.closed")]
    ChargeDisputeClosed,
    #[serde(rename = "charge.dispute.created")]
    ChargeDisputeCreated,
    #[serde(rename = "charge.dispute.funds_reinstated")]
    ChargeDisputeFundsReinstated,
    #[serde(rename = "charge.dispute.funds_withdrawn")]
    ChargeDisputeFundsWithdrawn,
    #[serde(rename = "charge.dispute.updated")]
    ChargeDisputeUpdated,
    #[serde(rename = "charge.refund.updated")]
    ChargeRefundUpdated,
    #[serde(rename = "checkout.session.completed")]
    CheckoutSessionCompleted,
    #[serde(rename = "coupon.created")]
    CouponCreated,
    #[serde(rename = "coupon.deleted")]
    CouponDeleted,
    #[serde(rename = "coupon.updated")]
    CouponUpdated,
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
    #[serde(rename = "customer.discount.created")]
    CustomerDiscountCreated,
    #[serde(rename = "customer.tax_id.created")]
    CustomerTaxIdCreated,
    #[serde(rename = "customer.tax_id.deleted")]
    CustomerTaxIdDeleted,
    #[serde(rename = "customer.tax_id.updated")]
    CustomerTaxIdUpdated,
    #[serde(rename = "customer.discount.deleted")]
    CustomerDiscountDeleted,
    #[serde(rename = "customer.discount.updated")]
    CustomerDiscountUpdated,
    #[serde(rename = "customer.source.created")]
    CustomerSourceCreated,
    #[serde(rename = "customer.source.deleted")]
    CustomerSourceDeleted,
    #[serde(rename = "customer.source.expiring")]
    CustomerSourceExpiring,
    #[serde(rename = "customer.source.updated")]
    CustomerSourceUpdated,
    #[serde(rename = "customer.subscription.created")]
    CustomerSubscriptionCreated,
    #[serde(rename = "customer.subscription.deleted")]
    CustomerSubscriptionDeleted,
    #[serde(rename = "customer.subscription.trial_will_end")]
    CustomerSubscriptionTrialWillEnd,
    #[serde(rename = "customer.subscription.updated")]
    CustomerSubscriptionUpdated,
    #[serde(rename = "file.created")]
    FileCreated,
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.deleted")]
    InvoiceDeleted,
    #[serde(rename = "invoice.finalized")]
    InvoiceFinalized,
    #[serde(rename = "invoice.marked_uncollectible")]
    InvoiceMarkedUncollectible,
    #[serde(rename = "invoice.payment_failed")]
    InvoicePaymentFailed,
    #[serde(rename = "invoice.payment_succeeded")]
    InvoicePaymentSucceeded,
    #[serde(rename = "invoice.payment_action_required")]
    InvoicePaymentActionRequired,
    #[serde(rename = "invoice.sent")]
    InvoiceSent,
    #[serde(rename = "invoice.upcoming")]
    InvoiceUpcoming,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
    #[serde(rename = "invoice.voided")]
    InvoiceVoided,
    #[serde(rename = "invoiceitem.created")]
    InvoiceitemCreated,
    #[serde(rename = "invoiceitem.deleted")]
    InvoiceitemDeleted,
    #[serde(rename = "invoiceitem.updated")]
    InvoiceitemUpdated,
    #[serde(rename = "issuing_authorization.created")]
    IssuingAuthorizationCreated,
    #[serde(rename = "issuing_authorization.request")]
    IssuingAuthorizationRequest,
    #[serde(rename = "issuing_authorization.updated")]
    IssuingAuthorizationUpdated,
    #[serde(rename = "issuing_card.created")]
    IssuingCardCreated,
    #[serde(rename = "issuing_card.updated")]
    IssuingCardUpdated,
    #[serde(rename = "issuing_cardholder.created")]
    IssuingCardholderCreated,
    #[serde(rename = "issuing_cardholder.updated")]
    IssuingCardholderUpdated,
    #[serde(rename = "issuing_dispute.created")]
    IssuingDisputeCreated,
    #[serde(rename = "issuing_dispute.updated")]
    IssuingDisputeUpdated,
    #[serde(rename = "issuing_transaction.created")]
    IssuingTransactionCreated,
    #[serde(rename = "issuing_transaction.updated")]
    IssuingTransactionUpdated,
    #[serde(rename = "issuing_settlement.created")]
    IssuingSettlementCreated,
    #[serde(rename = "issuing_settlement.updated")]
    IssuingSettlementUpdated,
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.payment_failed")]
    OrderPaymentFailed,
    #[serde(rename = "order.payment_succeeded")]
    OrderPaymentSucceeded,
    #[serde(rename = "order.updated")]
    OrderUpdated,
    #[serde(rename = "order_return.created")]
    OrderReturnCreated,
    #[serde(rename = "payment_intent.amount_capturable_updated")]
    PaymentIntentAmountCapturableUpdated,
    #[serde(rename = "payment_intent.created")]
    PaymentIntentCreated,
    #[serde(rename = "payment_intent.payment_failed")]
    PaymentIntentPaymentFailed,
    #[serde(rename = "payment_intent.requires_capture")]
    PaymentIntentRequiresCapture,
    #[serde(rename = "payment_intent.succeeded")]
    PaymentIntentSucceeded,
    #[serde(rename = "payment_method.attached")]
    PaymentMethodAttached,
    #[serde(rename = "payment_method.card_automatically_updated")]
    PaymentMethodCardAutomaticallyUpdated,
    #[serde(rename = "payment_method.detached")]
    PaymentMethodDetached,
    #[serde(rename = "payout.canceled")]
    PayoutCanceled,
    #[serde(rename = "payout.created")]
    PayoutCreated,
    #[serde(rename = "payout.failed")]
    PayoutFailed,
    #[serde(rename = "payout.paid")]
    PayoutPaid,
    #[serde(rename = "payout.updated")]
    PayoutUpdated,
    #[serde(rename = "person.created")]
    PersonCreated,
    #[serde(rename = "person.deleted")]
    PersonDeleted,
    #[serde(rename = "person.updated")]
    PersonUpdated,
    #[serde(rename = "plan.created")]
    PlanCreated,
    #[serde(rename = "plan.deleted")]
    PlanDeleted,
    #[serde(rename = "plan.updated")]
    PlanUpdated,
    #[serde(rename = "product.created")]
    ProductCreated,
    #[serde(rename = "product.deleted")]
    ProductDeleted,
    #[serde(rename = "product.updated")]
    ProductUpdated,
    #[serde(rename = "recipient.created")]
    RecipientCreated,
    #[serde(rename = "recipient.deleted")]
    RecipientDeleted,
    #[serde(rename = "recipient.updated")]
    RecipientUpdated,
    #[serde(rename = "reporting.report_run.failed")]
    ReportingReportRunFailed,
    #[serde(rename = "reporting.report_run.succeeded")]
    ReportingReportRunSucceeded,
    #[serde(rename = "reporting.report_type.updated")]
    ReportingReportTypeUpdated,
    #[serde(rename = "review.closed")]
    ReviewClosed,
    #[serde(rename = "review.opened")]
    ReviewOpened,
    #[serde(rename = "sigma.scheduled_query_run.created")]
    SigmaScheduledQueryRunCreated,
    #[serde(rename = "sku.created")]
    SkuCreated,
    #[serde(rename = "sku.deleted")]
    SkuDeleted,
    #[serde(rename = "sku.updated")]
    SkuUpdated,
    #[serde(rename = "source.canceled")]
    SourceCanceled,
    #[serde(rename = "source.chargeable")]
    SourceChargeable,
    #[serde(rename = "source.failed")]
    SourceFailed,
    #[serde(rename = "source.mandate_notification")]
    SourceMandateNotification,
    #[serde(rename = "source.refund_attributes_required")]
    SourceRefundAttributesRequired,
    #[serde(rename = "source.transaction.created")]
    SourceTransactionCreated,
    #[serde(rename = "source.transaction.updated")]
    SourceTransactionUpdated,
    #[serde(rename = "subscription_schedule.aborted")]
    SubscriptionScheduleAborted,
    #[serde(rename = "subscription_schedule.canceled")]
    SubscriptionScheduleCanceled,
    #[serde(rename = "subscription_schedule.completed")]
    SubscriptionScheduleCompleted,
    #[serde(rename = "subscription_schedule.created")]
    SubscriptionScheduleCreated,
    #[serde(rename = "subscription_schedule.expiring")]
    SubscriptionScheduleExpiring,
    #[serde(rename = "subscription_schedule.released")]
    SubscriptionScheduleReleased,
    #[serde(rename = "subscription_schedule.updated")]
    SubscriptionScheduleUpdated,
    #[serde(rename = "topup.canceled")]
    TopupCanceled,
    #[serde(rename = "topup.created")]
    TopupCreated,
    #[serde(rename = "topup.failed")]
    TopupFailed,
    #[serde(rename = "topup.reversed")]
    TopupReversed,
    #[serde(rename = "topup.succeeded")]
    TopupSucceeded,
    #[serde(rename = "transfer.created")]
    TransferCreated,
    #[serde(rename = "transfer.reversed")]
    TransferReversed,
    #[serde(rename = "transfer.updated")]
    TransferUpdated,
    #[serde(rename = "transfer.paid")]
    TransferPaid,
    #[serde(rename = "transfer.failed")]
    TransferFailed,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub object: Object,
    pub api_version: String,
    pub created: i64,
    pub data: EventData,
    pub livemode: bool,
    pub pending_webhooks: i64,
    pub request: Option<EventRequest>,
    #[serde(rename = "type")]
    pub event_type: EventType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventRequest {
    pub id: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventData {
    pub object: EventObject,
    pub previous_attributes: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged, rename_all = "snake_case")]
pub enum EventObject {
    ApplicationFees(ApplicationFees),
    ApplicationFeeRefund(ApplicationFeeRefunds),
    Authorization(Authorizations),
    Charge(Charge),
    CheckoutSessions(Sessions),
    Invoice(Invoice),
    InvoiceItem(InvoiceItems),
    Subscription(Subscription),
    File(File),
    Card(Card),
    CardHolder(CardHolders),
    IssuingCard(IssuingCard),
    IssuingDispute(IssuingDispute),
    IssuingTransaction(Transactions),
    Customer(Customer),
    Coupon(Coupon),
    Balance(Balance),
    Plan(Plans),
    Dispute(Dispute),
    Payout(Payout),
    Refund(Refund),
    Token(Tokens),
    BankAccount(BankAccount),
    Source(Source),
    Discount(Discount),
    Topup(Topup),
    Transfer(Transfer),
    SubscriptionItem(SubscriptionItems),
    SubscriptionSchedule(SubscriptionSchedules),
    PaymentIntent(PaymentIntent),
    PaymentMethod(PaymentMethods),
    Person(Persons),
    Review(Reviews),
    Order(Order),
    Product(Products),
    OrderReturn(OrderReturn),
    Sku(Sku),
}

impl Event {
    pub fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get(UrlPath::Events, vec![id], serde_json::Map::new())
    }

    pub fn list<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<List<Self>> {
        client.get(UrlPath::Events, vec![], param)
    }
}
