use axum::{Router, http::StatusCode, routing::get};

pub fn config_route() -> Router {
    return Router::new().route("/config", get(StatusCode::OK));
}
