use std::env;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::ResponseError;

struct AppState {
    pool: SqlitePool,
}

#[derive(Deserialize)]
struct Pagination {
    top: u32,
    bottom: u32,
}

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn new_boards_router() -> Result<Router, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:db.sqlite3".to_string());
    let pool = SqlitePool::connect(&db_url).await?;
    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(get_boards).post(new_board))
        .route("/{*id}", get(get_board))
        .with_state(Arc::new(state));

    Ok(app)
}

pub async fn get_boards(
    State(state): State<Arc<AppState>>,
    Query(params): Query<Pagination>,
) -> Result<impl IntoResponse, ResponseError> {
    let user: i64 = 0; // TODO: How to get this?

    let boards = sqlx::query!(
        r#"SELECT title, description FROM boards WHERE owner=?"#,
        user
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(())
}

pub async fn new_board() -> Result<impl IntoResponse, ResponseError> {
    Ok(())
}

pub async fn get_board() -> Result<impl IntoResponse, ResponseError> {
    Ok(())
}
