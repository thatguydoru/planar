use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

use crate::templates::ErrorTemplate;

macro_rules! from_error_to_string_impl {
    ($type:ty, $implementor:ty) => {
        impl From<$type> for $implementor {
            fn from(value: $type) -> Self {
                Self(value.to_string())
            }
        }
    };
    ($type:ty, $implementor:ty, $variant:ident) => {
        impl From<$type> for $implementor {
            fn from(value: $type) -> Self {
                Self::$variant(value.to_string())
            }
        }
    };
}

#[derive(Debug)]
pub struct ServeError(String);

impl Display for ServeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Serve: {}", self.0)
    }
}

from_error_to_string_impl!(std::io::Error, ServeError);
from_error_to_string_impl!(sqlx::Error, ServeError);
from_error_to_string_impl!(axum::Error, ServeError);

#[derive(Debug)]
pub enum ResponseError {
    Other(StatusCode),
    BadRequest(HashMap<String, String>),
    Internal(String),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::Other(code) => write!(f, "Http Error: {code}"),
            ResponseError::BadRequest(_) => write!(f, "Client Error: bad request"),
            ResponseError::Internal(reason) => write!(f, "Internal Error: {reason}"),
        }
    }
}

impl Error for ResponseError {}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        let status = match &self {
            ResponseError::Other(errcode) => *errcode,
            ResponseError::BadRequest(error_map) => todo!(),
            ResponseError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let template = ErrorTemplate { status }.render().unwrap();

        (status, Html(template)).into_response()
    }
}

from_error_to_string_impl!(sqlx::Error, ResponseError, Internal);

impl From<QueryError> for ResponseError {
    fn from(value: QueryError) -> Self {
        match value {
            QueryError::Value(_) => Self::Other(StatusCode::BAD_REQUEST),
            QueryError::Database(error) => Self::Internal(error.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum QueryError {
    Value(String),
    Database(sqlx::Error),
}

impl QueryError {
    pub fn value(message: &str) -> Self {
        Self::Value(message.to_string())
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            QueryError::Value(message) => write!(f, "Value: {message}"),
            QueryError::Database(error) => write!(f, "Database: {error}"),
        }
    }
}

impl Error for QueryError {}

impl From<sqlx::Error> for QueryError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}
