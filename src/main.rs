use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::Method,
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use well_written_server::schema;

async fn graphql_handler(
    schema: Extension<schema::AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint(&format!("{}/graphql", std::env::var("SERVER_URL").unwrap()))
            .finish(),
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let schema = Schema::build(schema::QueryRoot, EmptyMutation, EmptySubscription).finish();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .layer(cors)
        .route("/", get(|| async { "Hello, World!" }))
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!(
        "GraphiQL IDE: {}/graphql",
        std::env::var("SERVER_URL").unwrap()
    );

    let port = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
