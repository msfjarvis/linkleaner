use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Root2>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]

pub enum AMPResponse {
    Success(Root),
    Error(AMPError),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AMPError {
    pub error_message: String,
    pub result_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub canonical: Canonical,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canonical {
    pub url: String,
}
