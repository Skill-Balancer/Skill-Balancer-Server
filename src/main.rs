use crate::network::transition::Transition;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use dotenv::dotenv;
use network::profile::Profile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tower_http::services::ServeDir;

//importing routes and files.
mod config;
mod routes;
// importing models
mod env;
mod models;
mod network;
mod storage;

#[derive(Debug, Serialize, Deserialize)]
struct RecommendationResponse {
    recommendation: Value,
}
#[derive(Clone)]
struct AppState {
    profiles: Arc<RwLock<HashMap<String, Profile>>>,
    transitions: Arc<Mutex<Vec<Transition>>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState {
        profiles: Arc::new(RwLock::new(HashMap::new())),
        transitions: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .merge(routes::root::get_root())
        .nest_service("/models", ServeDir::new("data"))
        .merge(routes::config_route::config_route())
        .merge(routes::step_route::step_route())
        .merge(routes::all_config_route::all_config_route())
        .merge(routes::model::save_model_route())
        .merge(routes::model::load_model_route())
        .merge(routes::model::export_model_route())
        .merge(routes::model::list_saves_route())
        .merge(routes::model::list_exports_route())
        .with_state(state);

    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
