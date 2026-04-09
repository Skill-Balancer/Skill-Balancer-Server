use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

pub fn all_config_route() -> Router<AppState> {
    Router::new().route("/config/all", get(list_profiles))
}

async fn list_profiles(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.get_all_configs().await {
        Ok(configs) => (StatusCode::OK, Json(json!({ "configs": configs }))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to retrieve profiles" })),
        ),
    }
}
