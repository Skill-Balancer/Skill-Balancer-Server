use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Serialize, Deserialize)]
struct ProfilesJSON {
    name: String,
    description: Option<String>,
}

pub fn all_config_route() -> Router<AppState> {
    Router::new().route("/config/all", get(list_profiles))
}

async fn list_profiles(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_all_configs().await {
        Ok(configs) => (StatusCode::OK, Json(json!({ "configs": configs }))),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to retrieve profiles" })),
            );
        }
    }
}
