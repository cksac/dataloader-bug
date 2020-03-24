use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{
    http::{header, Method, Response},
    Filter,
};

mod graphql;

// #[tokio::main]
#[tokio::main(threaded_scheduler)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ::std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    let graphql = warp::path!("graphql")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|gql: GraphQLRequest| async move {
            let ctx = graphql::Context::new();
            let result = gql.execute(&graphql::ROOT_NODE, &ctx).await;
            Ok(warp::reply::json(&result)) as Result<_, Infallible>
        });

    let schema = warp::path!("graphql" / "schema.json")
        .and(warp::get())
        .map(|| {
            Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .body(graphql::SCHEMA_JSON.as_slice())
        });

    let graphiql = warp::path!("graphiql")
        .and(warp::get())
        .and(warp::header::<SocketAddr>("host"))
        .map(|addr: SocketAddr| {
            let html = graphiql_source(&format!("http://{}/graphql", addr));
            Ok(warp::reply::html(html))
        });

    let cors = warp::cors()
        .allow_origins(vec![format!("http://{}", addr).as_str()])
        .allow_credentials(true)
        .allow_headers(vec![header::CONTENT_TYPE])
        .allow_methods(vec![Method::POST]);

    let filters = graphql
        .or(schema)
        .or(graphiql)
        .with(&cors)
        .with(warp::log("GraphQL"));

    log::debug!("running");

    warp::serve(filters).run(addr).await;

    Ok(())
}
