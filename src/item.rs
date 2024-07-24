use serde::{Deserialize, Serialize};

use axum::{extract::Extension, Json};
use sqlx::{query_as, PgPool};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub async fn get_items(Extension(pool): Extension<Arc<PgPool>>) -> Json<Vec<Item>> {
    // TODO: sqlx実装
    // let items = query_as!(Item, "SELECT * FROM items")
    //     .fetch_all(&*pool)
    //     .await
    //     .expect("Failed to fetch items");

    let items = vec![
        Item {
            id: 1,
            name: "Jone Doe".to_string(),
            description: "this is sample text".to_string(),
        },
        Item {
            id: 1,
            name: "Jone Doe".to_string(),
            description: "this is sample text".to_string(),
        },
        Item {
            id: 1,
            name: "Jone Doe".to_string(),
            description: "this is sample text".to_string(),
        },
    ];
    Json(items)
}
