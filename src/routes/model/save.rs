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
    return Router::new().route("/save/{name}", get(handle_save_model));
}

async fn handle_save_model(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let profiles = state.profiles.lock().await;

    let profile = match profiles.iter().find(|p| p.name == name) {
        Some(val) => val,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Could not find profile with id {}", name),
                })),
            );
        }
    };

    let checkpoint = CheckPoint::new(profile.name.clone());
    checkpoint.save(profile.trainer.model.clone());
    drop(profiles);

    (
        StatusCode::OK,
        Json(json!({
            "message": format!("Model saved"),
            "url": checkpoint.to_url(),
        })),
    )
}
