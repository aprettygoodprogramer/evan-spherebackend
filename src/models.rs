use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
}
