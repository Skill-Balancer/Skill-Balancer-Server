use axum::Router;
use dotenv::dotenv;
use std::env;

//importing routes and files.
mod config;
mod routes;

// importing models
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().merge(routes::root::get_root());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
