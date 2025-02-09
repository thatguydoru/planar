use axum::http::StatusCode;
use rinja::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub status: StatusCode,
}
