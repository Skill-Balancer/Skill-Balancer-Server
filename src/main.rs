

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use dotenv::dotenv;
use std::{collections::HashMap, sync::Arc};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use models::profile::Profile;
use tokio::sync::{Mutex, RwLock};
use crate::models::transition::Transition;

//importing routes and files.
mod config;
mod routes;
// importing models
mod models;


#[derive(Debug, Serialize, Deserialize)]
struct RecommendationResponse {
    recommendation: Value,
}
#[derive(Clone)]
struct AppState {
    profiles: Arc<RwLock<HashMap<String, Profile>>>,
    transitions: Arc<Mutex<Vec<Transition>>>,
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.message
        }));
        (self.status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState{
        profiles: Arc::new(RwLock::new(HashMap::new())),
        transitions: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .merge(routes::root::get_root())
        .merge(routes::config_route::config_route())
        .merge(routes::step_route::step_route())
        .merge(routes::all_config_route::all_config_route()).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}
