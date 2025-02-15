use std::error::Error;
use std::fmt::{self, Display, Formatter};

use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use rinja::Template;

use crate::templates::ErrorTemplate;

macro_rules! from_error_to_internal_impl {
    ($type:ty, $implementor:ty) => {
        impl From<$type> for $implementor {
            fn from(value: $type) -> Self {
                Self::Internal(value.into())
            }
        }
    };
}

#[derive(Debug)]
pub enum AppError {
    NoRoute,
    Internal(Box<dyn Error>),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NoRoute => write!(f, "Client: No route matches."),
            AppError::Internal(error) => write!(f, "Other: {error}"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::NoRoute => StatusCode::NOT_FOUND,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = ErrorTemplate { status }.render().unwrap();

        (status, Html(tmpl)).into_response()
    }
}

from_error_to_internal_impl!(std::io::Error, AppError);
from_error_to_internal_impl!(sqlx::Error, AppError);
from_error_to_internal_impl!(axum::Error, AppError);
