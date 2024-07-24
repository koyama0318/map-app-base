use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn establish_connection(url: &String) -> PgPool {
    // TODO: sqlx実装
    PgPool::connect(&url)
        .await
        .expect("Failed to connect to database")
}
