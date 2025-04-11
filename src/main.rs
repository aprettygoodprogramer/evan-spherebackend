use dotenv::dotenv;
use std::env;

use chrono::Duration as ChronoDuration;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    pub db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".into());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET])
        .allow_headers(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    //Test Database Connection
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => println!("✅ Database connection successful!"),
        Err(e) => {
            eprintln!("❌ Database connection failed: {}", e);
            std::process::exit(1);
        }
    }
    let app_state: AppState = AppState { db_pool: pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(app_state)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
