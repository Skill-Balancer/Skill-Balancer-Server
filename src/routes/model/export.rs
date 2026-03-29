use crate::{AppState, storage::model::CheckPoint};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use burn::module::Module;
use serde_json::json;

pub fn export_model_route() -> Router<AppState> {
    return Router::new().route("/export/{name}", get(handle_export_model));
}

async fn handle_export_model(
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

    let devices = profile.trainer.model.devices();

    let device = match devices.first() {
        Some(val) => val,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("Could not find backend for this config, this error should never print!"),
                })),
            );
        }
    };
    let checkpoint = CheckPoint::new(profile.name.clone());
    let res = checkpoint.export(profile.trainer.model.clone(), &device);

    match res {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": format!("Model exported"),
                "url": checkpoint.to_export_url(),
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Failed to export model: {}", e)})),
        ),
    }
}
