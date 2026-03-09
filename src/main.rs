mod profile;

use axum::{Router, response::Html, routing::get};
use axum::Router;
use dotenv::dotenv;
use std::env;
use serde_json::{json, Value};

//importing routes and files.
mod config;
mod routes;

// importing models
mod models;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transition {
    id: String,
    session_id: String,
    step: i64,
    state: Value,
    action: Value,
    reward: f64,
    next_state: Value,
    done: bool,
    timestamp: String,
}




#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::<()>::new()
        .merge(routes::root::get_root())
        .merge(routes::config_route::config_route())
        .merge(routes::step_route::step_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
