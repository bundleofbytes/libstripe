#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Address {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub town: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}
