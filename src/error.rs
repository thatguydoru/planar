use std::error::Error;
use std::fmt::{self, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

use crate::templates::ErrorTemplate;

#[derive(Debug)]
pub enum AppError {
    Http(StatusCode),
    Internal,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Http(code) => write!(f, "{code}"),
            AppError::Internal => write!(f, "500 Internal Error"),
        }
    }
}

impl Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Http(errcode) => *errcode,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let template = ErrorTemplate { status }.render().unwrap();

        (status, Html(template)).into_response()
    }
}

#[derive(Debug)]
pub enum ModelError {
    Value(String),
    Database(sqlx::Error),
}

impl ModelError {
    pub fn value(message: &str) -> Self {
        Self::Value(message.to_string())
    }
}

impl Display for ModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::Value(message) => write!(f, "{message}"),
            ModelError::Database(error) => write!(f, "{error}"),
        }
    }
}

impl Error for ModelError {}

impl From<sqlx::Error> for ModelError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}
