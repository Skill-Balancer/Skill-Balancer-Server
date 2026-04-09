use crate::{AppState, storage::model::list_checkpoints};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

pub fn list_checkpoints_route() -> Router<AppState> {
    return Router::new().route("/checkpoint", get(handle_saves_model));
}

async fn handle_saves_model(State(state): State<AppState>) -> impl IntoResponse {
    match state.profile.lock().await.as_ref() {
        Some(profile) => {
            let checkpoints = list_checkpoints(profile.name.clone());
            let checkpoints_info: Vec<_> = checkpoints
                .iter()
                .filter(|checkpoint| checkpoint.config_name == profile.name)
                .map(|checkpoint| {
                    json!({
                        "model_id": checkpoint.id,
                        "url": checkpoint.to_url(),
                    })
                })
                .collect();
            (
                StatusCode::OK,
                Json(json!({
                    "message": format!("List of checkpoints for profile {}", profile.name),
                    "checkpoints": checkpoints_info,
                })),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "message": format!("Could not find profile"),
            })),
        ),
    }
}
