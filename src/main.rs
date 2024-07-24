mod auth;
mod db;
mod item;

use axum::{routing::get, Extension, Router};
//use db::establish_connection;
use dotenv::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = env::var("ADDR").expect("ADDR must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    //let pool = establish_connection(&database_url).await;

    println!("Starting server on {}:{}", addr, port);
    println!("Connecting to database at {}", database_url);

    let app = Router::new().route("/", get(hello)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            //.layer(Extension(Arc::new(pool)))
            .layer(Extension(secret))
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
