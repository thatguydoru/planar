mod error;
mod routes;
mod templates;

use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use error::{ResponseError, ServeError};
use routes::{new_boards_router, ping};

#[tokio::main]
async fn main() -> Result<(), ServeError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let socket_addr = ("127.0.0.1", 8000);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    let board_app = new_boards_router().await?;

    let app = Router::new()
        .route("/ping", get(ping))
        .nest("/app", board_app)
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/third-party", ServeDir::new("node_modules"))
        .fallback(|| async { ResponseError::Other(StatusCode::NOT_FOUND) })
        .layer(TraceLayer::new_for_http());

    println!("Served at: http://{}:{}", socket_addr.0, socket_addr.1);
    axum::serve(listener, app).await?;

    Ok(())
}
