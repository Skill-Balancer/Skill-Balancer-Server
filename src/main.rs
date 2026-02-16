use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(Html("
        <h1>Hello!</h1>
        <p>Hi from Rust</p>"
    )));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
