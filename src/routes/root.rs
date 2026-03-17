use axum::{Router, response::Html, routing::get};

pub fn get_root() -> Router {
    return Router::new().route(
        "/",
        get(Html(
            "
        <h1>Hello!</h1>
        <p>Hi from Rust</p>",
        )),
    );
}