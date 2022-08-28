use juniper::graphql_object;
use ::serde::Deserialize;

// ############################################################################
// Parser
// ############################################################################
#[derive(Clone, Copy, Debug)]
pub struct Context;
impl juniper::Context for Context {}

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
