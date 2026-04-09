use crate::{AppState, storage::model::CheckPoint};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde_json::json;

pub fn checkpoint_route() -> Router<AppState> {
    return Router::new().route("/checkpoint/{id}", get(handle_checkpoint));
}

async fn handle_checkpoint(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.profile.lock().await.as_ref() {
        Some(profile) => {
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
