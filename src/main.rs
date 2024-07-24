mod auth;
mod db;

use axum::{routing::get, Extension, Router};
use db::establish_connection;
use dotenv::dotenv;
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = env::var("ADDR").expect("ADDR must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let database_url = env::var("DB_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection(&database_url).await;

    println!("Starting server on {}:{}", addr, port);
    println!("Connecting to database at {}", database_url);

    let app = Router::new().route("/", get(hello)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(Extension(Arc::new(pool)))
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

// use serde::{Deserialize, Serialize};

// use axum::{extract::Extension, Json};
// use sqlx::PgPool;
// use std::sync::Arc;

// tokio = { version = "1.38.1", features = ["full"] }
// axum = { version = "0.7.5", features = ["macros", "json"] }
// tower = "0.4.13"
// tower-http = { version = "0.5.2", features = ["trace"] }
// sqlx = { version = "0.7.4", features = ["postgres", "runtime-tokio-native-tls", "macros"] }
// serde = { version = "1.0.204", features = ["derive"] }
// jsonwebtoken = "9.3.0"
// dotenv = "0.15"
// hyper = { version = "0.14", features = ["full"] }
// #[derive(Serialize, Deserialize)]
// pub struct Item {
//     pub id: i32,
//     pub name: String,
//     pub description: String,
// }

// pub async fn get_items(Extension(pool): Extension<Arc<PgPool>>) -> Json<Vec<Item>> {
//     let items = sqlx::query_as!(Item, "SELECT * FROM items")
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch items");
// }

// use axum::body::Body;
// use axum::{http::Request, middleware::Next, response::Response};
// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
// use serde::Deserialize;
// use std::env;

// #[derive(Debug, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }

// pub async fn jwt_middleware(
//     req: Request<Body>,
//     next: Next,
// ) -> Result<Response, axum::http::StatusCode> {
//     let secret = env::var("JWT_SECRET").expect("DATABASE_URL must be set");
//     if let Some(authen_header) = req.headers().get("Authorization") {
//         if let Ok(authen_str) = authen_header.to_str() {
//             if authen_str.starts_with("Bearer ") {
//                 let token = &authen_str[7..];
//                 let validation = Validation::new(Algorithm::HS256);
//                 let token_data = decode::<Claims>(
//                     &token,
//                     &DecodingKey::from_secret(secret.as_ref()),
//                     &validation,
//                 );

//                 if token_data.is_ok() {
//                     return Ok(next.run(req).await);
//                 }
//             }
//         }
//     }

//     Err(axum::http::StatusCode::UNAUTHORIZED)
// }
