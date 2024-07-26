use crate::adapter::gateways::db_connection::establish_connection;
use dotenv::dotenv;
use std::env;
use tracing::info;

pub struct AppConfig {
    pub addr: String,
    pub port: String,
    pub secret: String,
    // pub pool: sqlx::PgPool,
}

impl AppConfig {
    pub fn new(addr: String, port: String, secret: String) -> Self {
        Self {
            addr,
            port,
            secret,
            // pool,
        }
    }
}

pub async fn load_config() -> AppConfig {
    dotenv().ok();
    let addr = env::var("ADDR").expect("ADDR must be set");
    let port = env::var("PORT").expect("PORT must be set");
    info!(server.addr=?addr, server.port=?port, "starting server");

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    info!(db.url=?database_url, "connecting to db");

    // let pool = establish_connection(&database_url)
    //     .await
    //     .expect("Failed to connect to db");
    // info!("connected to db");

    AppConfig::new(addr, port, secret)
}
