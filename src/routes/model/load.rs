use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use burn::module::Module;
use serde_json::json;

use crate::{AppState, storage::model::CheckPoint};

pub fn load_model_route() -> Router<AppState> {
    return Router::new().route("/load/{id}", get(handle_load_model));
}

async fn handle_load_model(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let (model, device, name) = {
        let guard = state.profile.lock().await;
        let profile = match guard.as_ref() {
            Some(val) => val,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "message": format!("Could not find profile with id {}", id) })),
                );
            }
        };
        let devices = profile.trainer.model.devices();
        let device = match devices.first() {
            Some(d) => d.clone(),
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "No backend found" })),
                );
            }
        };
        (profile.trainer.model.clone(), device, profile.name.clone())
    };

    let checkpoint = CheckPoint::new(name, id);
    match checkpoint.load(model, &device) {
        Ok(loaded_model) => {
            if let Some(profile) = state.profile.lock().await.as_mut() {
                profile.trainer.model = loaded_model;
            }
            (
                StatusCode::OK,
                Json(json!({ "message": "Model loaded successfully" })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": format!("Failed to load model: {}", e) })),
        ),
    }
}
