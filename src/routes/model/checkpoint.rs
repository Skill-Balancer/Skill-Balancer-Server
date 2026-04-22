use crate::{
    AppState,
    storage::model::{CheckPoint, list_checkpoints},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

pub fn checkpoint_route() -> Router<AppState> {
    Router::new()
        .route("/checkpoint", get(list_all_checkpoints))
        .merge(Router::new().route("/checkpoint/{id}", post(create_checkpoint)))
}

async fn create_checkpoint(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.profile.lock().await.as_ref() {
        Some(profile) => {
            match state.config_tx.send(profile.name.clone()) {
                Ok(_) => (),
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": format!("Failed to send config update: {}", e)})),
                    );
                }
            }
            let checkpoint = CheckPoint::new(profile.name.clone(), id);
            checkpoint.save(profile.trainer.model.clone());
            (
                StatusCode::OK,
                Json(json!({
                    "message": format!("Model checkpoint created"),
                    "url": checkpoint.to_url(),
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

async fn list_all_checkpoints(State(state): State<AppState>) -> impl IntoResponse {
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
