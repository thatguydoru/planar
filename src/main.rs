mod errors;
mod models;

use std::io;

use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use rinja::Template;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use errors::AppError;
use models::{Card, Column};

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let socket_addr = ("127.0.0.1", 8000);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    let app = Router::new()
        .route("/boards", get(boards))
        .route("/cards", get(cards))
        .route("/ping", get(ping))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(|| async { AppError::NotFound })
        .layer(TraceLayer::new_for_http());

    println!("Served at: http://{}:{}", socket_addr.0, socket_addr.1);
    axum::serve(listener, app).await
}

async fn ping() -> &'static str {
    "pong"
}

#[derive(Template)]
#[template(path = "board/index.html")]
struct BoardIndexTemplate {
    columns: Vec<Column>,
}

async fn boards() -> impl IntoResponse {
    let template = BoardIndexTemplate {
        columns: vec![Column {
            id: 1,
            title: "column 1".to_string(),
        }],
    };

    Html(template.render().unwrap())
}

async fn cards() -> impl IntoResponse {}
