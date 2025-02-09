use std::error::Error;
use std::fmt::{self, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

use crate::templates::ErrorTemplate;

#[derive(Debug)]
pub enum AppError {
    Http(StatusCode),
    Internal(ErrorKind),
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Http(code) => write!(f, "{code}"),
            AppError::Internal(kind) => write!(f, "{kind}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Http(errcode) => *errcode,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let template = ErrorTemplate { status }.render().unwrap();

        (status, Html(template)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::Internal(ErrorKind::DatabaseError(value))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    ValueError(String),
    DatabaseError(sqlx::Error),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::ValueError(message) => write!(f, "{message}"),
            ErrorKind::DatabaseError(error) => todo!(),
        }
    }
}
