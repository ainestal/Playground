use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::any()
        .and(warp::header("user-agent"))
        .and(warp::header("authorization"))
        .map(|agent: String, auth: String| {
            warp::reply::html(format!("agent: {}. auth: {}", agent, auth))
        });

    let hi = warp::path("a")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            warp::reply::html(format!("Hello {}, whose agent is {}", param, agent))
        })
        .or(routes);

    warp::serve(hi).run(([127, 0, 0, 1], 8000)).await;
}
