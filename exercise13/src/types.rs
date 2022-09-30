use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Wraph)]
pub struct Ticker {
    pub error: Vec<Value>,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Wraph)]
pub struct Result {
    #[serde(rename = "XXBTZUSD")]
    pub xxbtzusd: Vec<(i64, String, String, String, String, String, String, i64)>,
    pub last: i64,
}
