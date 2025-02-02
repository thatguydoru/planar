mod errors;

use std::io;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use rinja::Template;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use errors::AppError;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let socket_addr = ("127.0.0.1", 8000);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;

    let app = Router::new()
        .route("/board", get(show_board))
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
#[template(path = "board.html")]
struct BoardTemplate;

async fn show_board() -> Html<String> {
    let template = BoardTemplate.render().unwrap();

    Html(template)
}
