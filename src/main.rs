mod auth;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = env::var("ADDR").unwrap();
    let port = env::var("PORT").unwrap();

    let app = Router::new().route("/", get(hello)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(axum::middleware::from_fn(auth::jwt_middleware)),
    );

    let addr = format!("{}:{}", addr, port)
        .parse::<SocketAddr>()
        .expect("Invalid address");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "hello"
}
