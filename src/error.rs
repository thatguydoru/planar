use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::io;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rinja::Template;

use crate::utils::render_now;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub kind: Option<ErrorKind>,
}

impl Error {
    pub fn err_to_other(message: &str, error: Box<dyn StdError>) -> Self {
        Self {
            message: message.to_string(),
            kind: Some(ErrorKind::Other(error)),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Some(kind) => write!(f, "{}\nCaused by: {kind:?}", self.message),
            None => write!(f, "{}", self.message),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.kind.as_ref().map(|kind| match kind {
            ErrorKind::Sqlx(err) => err,
            ErrorKind::Other(err) => err.as_ref(),
        })
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            render_now(InternalErrorPage),
        )
            .into_response()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    Sqlx(sqlx::Error),
    Other(Box<dyn StdError>),
}

macro_rules! other_error_impl {
    ($type:ty, $message:literal) => {
        impl From<$type> for Error {
            fn from(value: $type) -> Self {
                Self::err_to_other($message, value.into())
            }
        }
    };
}

other_error_impl!(io::Error, "bad I/O");
other_error_impl!(rinja::Error, "bad render");

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self {
            message: "bad sqlx".to_string(),
            kind: Some(ErrorKind::Sqlx(value)),
        }
    }
}

#[derive(Template)]
#[template(path = "500.html")]
struct InternalErrorPage;
