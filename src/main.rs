mod error;
mod queries;
mod routes;
mod templates;

use std::io;

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use error::AppError;
use routes::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let socket_addr = ("127.0.0.1", 8000);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    let app = Router::new()
        .route("/boards", get(boards))
        .route("/columns", get(columns))
        .route("/cards", get(cards))
        .route("/ping", get(ping))
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/third-party", ServeDir::new("node_modules"))
        .fallback(|| async { AppError::Http(StatusCode::NOT_FOUND) })
        .layer(TraceLayer::new_for_http());

    println!("Served at: http://{}:{}", socket_addr.0, socket_addr.1);
    axum::serve(listener, app).await
}
