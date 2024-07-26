use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}

// impl Item {
//     pub async fn get_items(Extension(pool): Extension<Arc<PgPool>>) -> Json<Vec<Item>> {
//         let items = query_as!(Item, "SELECT * FROM company1.items")
//             .fetch_all(&*pool)
//             .await
//             .expect("Failed to fetch items");

//         Json(items)
//     }
// }
