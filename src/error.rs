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
    FormHtml(Html<String>),
    Internal(String),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::Other(code) => write!(f, "Http Error: {code}"),
            ResponseError::FormHtml(html) => write!(f, "Client Error: {}", html.0),
            ResponseError::Internal(reason) => write!(f, "Internal Error: {reason}"),
        }
    }
}

impl Error for ResponseError {}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            ResponseError::FormHtml(html) => (StatusCode::BAD_REQUEST, html).into_response(),
            ResponseError::Internal(_) => {
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let tmpl = ErrorTemplate { status }.render().unwrap();

                (status, tmpl).into_response()
            }
            ResponseError::Other(status) => {
                let tmpl = ErrorTemplate { status }.render().unwrap();

                (status, tmpl).into_response()
            }
        }
    }
}

from_error_to_string_impl!(sqlx::Error, ResponseError, Internal);
