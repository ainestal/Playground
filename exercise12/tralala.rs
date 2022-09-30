use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Ticker = Vec<Ticker2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticker2 {
    pub volume: String,
    pub last: String,
    pub percent_change_24: Option<String>,
    pub timestamp: String,
    pub bid: String,
    pub vwap: String,
    pub high: String,
    pub pair: String,
    pub low: String,
    pub open_24: String,
    pub ask: String,
    pub open: String,
}
