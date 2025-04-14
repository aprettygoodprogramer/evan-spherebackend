mod models;
use sqlx::{PgPool, postgres::PgPoolOptions};

async fn retrieve_post(db_pool: &PgPool, id: i32) -> Result<models::Post, sqlx::Error> {
    let post = sqlx::query_as!(models::Post, "SELECT id FROM posts WHERE id = $1", id)
        .fetch_one(db_pool)
        .await?;
    Ok(post)
}
