use juniper::{graphql_object, FieldError};

use crate::parser::{Context, Ticker};

// ############################################################################
// Connnector
// ############################################################################

/// Fetch a URL and return the response body text.
async fn request(url: String) -> Result<String, FieldError> {
    Ok(reqwest::get(&url).await?.text().await?)
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn ticker() -> Ticker {
        let body = request("https://api.kraken.com/0/public/Ticker?pair=XBTUSD".to_string())
            .await
            .unwrap();
        let ticker: Ticker = serde_json::from_str(&body).unwrap();

        ticker
    }
}
