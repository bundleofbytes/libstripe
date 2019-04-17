use crate::resources::common::object::Object;

#[derive(Debug, Deserialize)]
pub struct Review {
    pub id: String,
    pub object: Object,
    pub charge: String,
    pub created: i64,
    pub livemode: bool,
    pub open: bool,
    pub reason: Reason
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Reason {
    Rule,
    Manual,
    Approved,
    Refunded,
    RefundedAsFraud,
    Disputed
}
