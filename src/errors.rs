use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    status: StatusCode,
}

pub enum AppError {
    NotFound,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let template = ErrorTemplate { status }.render().unwrap();

        (status, Html(template)).into_response()
    }
}
