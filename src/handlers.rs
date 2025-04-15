use crate::models::{AppState, Post};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::query_as;

pub async fn get_post_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, (axum::http::StatusCode, String)> {
    let post = query_as::<_, Post>(
        r#"
        SELECT id, title, content, slug, created_at
        FROM posts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(post))
}

pub async fn get_all_posts_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<Post>>, (axum::http::StatusCode, String)> {
    let posts = query_as::<_, Post>(
        r#"
        SELECT id, title, content, slug, created_at
        FROM posts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db_pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(posts))
}

pub async fn get_post_by_slug_handler(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Post>, (axum::http::StatusCode, String)> {
    let post = query_as::<_, Post>(
        r#"
        SELECT id, title, content, slug, created_at
        FROM posts
        WHERE slug = $1
        "#,
    )
    .bind(slug)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(post))
}
