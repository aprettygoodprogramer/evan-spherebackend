use sqlx::FromRow;
use sqlx::PgPool;

use serde::{Deserialize, Serialize};
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
