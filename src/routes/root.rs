use crate::AppState;
use askama::Template;
use axum::{Router, response::Html, routing::get};

#[derive(Template)]
#[template(path = "root.html")]
pub struct HelloTemplate<'a> {
    pub _name: &'a str,
}
pub fn get_root() -> Router<AppState> {
    let body = HelloTemplate { _name: "World" }.render().unwrap();
    return Router::new().route("/", get(Html(body)));
}
