use anyhow::bail;
use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

pub async fn establish_connection(url: &String) -> Result<Pool<Postgres>> {
    match PgPool::connect(&url).await {
        Ok(pool) => Ok(pool),
        Err(e) => bail!("Failed to connect to database: {}", e),
    }
}
