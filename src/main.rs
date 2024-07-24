mod auth;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(axum::middleware::from_fn(auth::jwt_middleware)),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "hello"
}
