mod adapter;
mod auth;
mod db;
mod domain;
mod item;
mod usecase;

use adapter::controller::v1::{destinations_router, drivers_router, routes_router, users_router};
use axum::{Extension, Router};
use db::establish_connection;
use dotenv::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;

// ロガーの初期化
fn tracing_initialize() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = env::var("ADDR").expect("ADDR must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let pool = establish_connection(&database_url).await;

    tracing_initialize();

    info!(server.addr=?addr, server.port=?port, "starting server");
    info!(db.url=?database_url, "connecting to db");

    let app = Router::new()
        .nest("/users", users_router::users_router())
        .nest("/drivers", drivers_router::drivers_router())
        .nest("/destinations", destinations_router::destinations_router())
        .nest("/routes", routes_router::routes_router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(Arc::new(pool)))
                .layer(Extension(secret))
                .layer(axum::middleware::from_fn(auth::jwt_middleware)),
        );

    let addr = format!("{}:{}", addr, port)
        .parse::<SocketAddr>()
        .expect("Invalid address");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
