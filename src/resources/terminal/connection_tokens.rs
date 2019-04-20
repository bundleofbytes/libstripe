use crate::resources::common::object::Object;
use serde::Serialize;
use crate::Client;
use crate::resources::common::path::UrlPath;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionTokens {
    pub object: Object,
    pub secret: String,
}

#[derive(Default, Debug, Serialize)]
pub struct ConnectionTokensParam<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_account: Option<&'a str>,
}

impl ConnectionTokens {

    pub fn create<S: Serialize>(client: &Client, param: S) -> crate::Result<Self> {
        client.post(UrlPath::TerminalConnectionTokens, vec![], param)
    }

}