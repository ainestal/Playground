use json_typegen::json_typegen;
use juniper::{graphql_object, FieldError};
use serde::Deserialize;

// ############################################################################
// Connnector
// ############################################################################

json_typegen!("Ticker", "data/output.json");

#[derive(Deserialize)]
struct MyTicker {
    tick: Ticker,
}

#[graphql_object(context = Context)]
impl MyTicker {
    fn tick(&self) -> MyTicker {
        MyTicker {
            tick: self.tick.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Context;
impl juniper::Context for Context {}

/// Fetch a URL and return the response body text.
async fn request(url: String) -> Result<String, FieldError> {
    Ok(reqwest::get(&url).await?.text().await?)
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn ticker() -> MyTicker {
        let body = request("https://www.bitstamp.net/api/v2/ticker/BTCUSD/".to_string())
            .await
            .unwrap();
        let ticker: MyTicker = MyTicker {
            tick: serde_json::from_str(&body).unwrap(),
        };

        ticker
    }
}
