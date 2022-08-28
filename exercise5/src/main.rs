use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldError, RootNode};
use serde::Deserialize;
use warp::{http::Response, Filter};

#[derive(Clone, Copy, Debug)]
pub struct Context;
impl juniper::Context for Context {}

// ############################################################################
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub error: Vec<String>,
    pub result: TickerResult,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerResult {
    #[serde(rename = "XXBTZUSD")]
    pub xxbtzusd: Xxbtzusd,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xxbtzusd {
    pub a: Vec<String>,
    pub b: Vec<String>,
    pub c: Vec<String>,
    pub v: Vec<String>,
    pub p: Vec<String>,
    pub t: Vec<i32>,
    pub l: Vec<String>,
    pub h: Vec<String>,
    pub o: String,
}

#[graphql_object(context = Context)]
impl Ticker {
    fn error(&self) -> Vec<String> {
        self.error.clone()
    }
    fn result(&self) -> TickerResult {
        self.result.clone()
    }
}

#[graphql_object(context = Context)]
impl TickerResult {
    fn xxbtzusd(&self) -> Xxbtzusd {
        self.xxbtzusd.clone()
    }
}

#[graphql_object(context = Context)]
impl Xxbtzusd {
    fn a(&self) -> Vec<String> {
        self.a.clone()
    }
    fn b(&self) -> Vec<String> {
        self.b.clone()
    }
    fn c(&self) -> Vec<String> {
        self.c.clone()
    }
    fn v(&self) -> Vec<String> {
        self.v.clone()
    }
    fn p(&self) -> Vec<String> {
        self.p.clone()
    }
    fn t(&self) -> Vec<i32> {
        self.t.clone()
    }
    fn l(&self) -> Vec<String> {
        self.l.clone()
    }
    fn h(&self) -> Vec<String> {
        self.h.clone()
    }
    fn o(&self) -> String {
        self.o.clone()
    }
}

#[derive(Clone, Copy, Debug)]
struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn ticker() -> Ticker {
        let ticker: Ticker = retrieve_ticker().await;
        ticker
    }

    /// Fetch a URL and return the response body text.
    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}

async fn retrieve_ticker() -> Ticker {
    let body = reqwest::get("https://api.kraken.com/0/public/Ticker?pair=XBTUSD")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let ticker: Ticker = serde_json::from_str(&body).unwrap();
    ticker
}

// ############################################################################

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warp_async");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>",
            )
    });

    log::info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(|| Context);
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
