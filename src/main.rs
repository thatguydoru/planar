mod auth;
mod error;
mod utils;

use std::env;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::error::Error as AppError;

pub type Result<T, E = AppError> = std::result::Result<T, E>;
pub type AppState = Arc<State>;

pub struct State {
    pub db: SqlitePool,
}

impl State {
    async fn new() -> Result<Self> {
        let url = env::var("DATABASE_URL").expect("must set db url");

        Ok(Self {
            db: SqlitePool::connect(&url).await?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("PLANAR_LOG"))
        .compact()
        .init();

    let state = Arc::new(State::new().await?);
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;
    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await?;

    Ok(())
}
