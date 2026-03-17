use axum::{Router, response::Html, routing::get};
use crate::AppState;

pub fn get_root() -> Router<AppState> {
    return Router::new().route(
        "/",
        get(Html(
            "
        <h1>Hello!</h1>
        <p>Hi from Rust</p>",
        )),
    );
}
