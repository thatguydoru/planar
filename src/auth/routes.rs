use axum::extract::State;
use axum::response::{Html, Response};
use axum::routing::get;
use axum::{Form, Router};

use super::forms::SignupForm;
use super::templates::SignupPartial;
use crate::utils::render_now;
use crate::{AppState, Result};

pub fn new_router(state: AppState) -> Router {
    Router::new()
        .route("/signup", get(signup_partial).post(signup))
        .with_state(state)
}

async fn signup_partial() -> Html<String> {
    render_now(SignupPartial::default())
}

async fn signup(State(state): State<AppState>, Form(form): Form<SignupForm>) -> Result<Response> {
    todo!()
}
