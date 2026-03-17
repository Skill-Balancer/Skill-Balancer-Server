use axum::{Router, http::StatusCode, routing::get};

pub fn step_route() -> Router {
    return Router::new().route("/step", get(StatusCode::OK));
}
