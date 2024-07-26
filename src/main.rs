mod adapter;
mod application;
mod config;
mod domain;

use crate::config::load_config;
use adapter::controller::v1::{drivers_router, places_router, routes_router, users_router};
use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // 環境変数
    let config = load_config().await;

    // ロガーの初期化
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .nest("/users", users_router::users_router())
        .nest("/drivers", drivers_router::drivers_router())
        .nest("/places", places_router::places_router())
        .nest("/routes", routes_router::routes_router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                // .layer(Extension(Arc::new(pool)))
                .layer(Extension(config.secret)), // .layer(axum::middleware::from_fn(auth::jwt_middleware)),
        );

    let addr = format!("{}:{}", config.addr, config.port)
        .parse::<SocketAddr>()
        .expect("Invalid address");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
