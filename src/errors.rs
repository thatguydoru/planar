use std::error::Error;
use std::fmt::{self, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

use crate::templates::ErrorTemplate;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Internal,
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "page not found"),
            AppError::Internal => write!(f, "internal error"),
        }
    }
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

#[derive(Debug)]
pub struct ValueError {
    pub message: String,
}

impl Error for ValueError {}

impl Display for ValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ValueError: {}", self.message)
    }
}

impl<'a> From<&'a str> for ValueError {
    fn from(value: &'a str) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
