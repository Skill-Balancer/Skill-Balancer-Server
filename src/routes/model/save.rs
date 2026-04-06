use crate::{AppState, storage::model::CheckPoint};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde_json::json;

pub fn save_model_route() -> Router<AppState> {
    return Router::new().route("/save/{id}", get(handle_save_model));
}

async fn handle_save_model(
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
                    "message": format!("Model saved"),
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
