use axum::response::{Html, IntoResponse};

pub async fn ping() -> &'static str {
    "pong"
}

pub async fn boards() -> impl IntoResponse {}

pub async fn cards() -> impl IntoResponse {}

pub async fn columns() -> impl IntoResponse {}
