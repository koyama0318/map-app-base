use sqlx::PgPool;

pub async fn establish_connection(url: &String) -> PgPool {
    PgPool::connect(&url)
        .await
        .expect("Failed to connect to database")
}
