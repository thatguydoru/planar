use std::fmt::{self, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;
use thiserror::Error;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    status: StatusCode,
}

#[derive(Debug, Error)]
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

#[derive(Debug, Error)]
pub struct ValueError {
    pub message: String,
}

impl<'a> From<&'a str> for ValueError {
    fn from(value: &'a str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl Display for ValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ValueError: {}", self.message)
    }
}
