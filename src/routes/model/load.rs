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
    return Router::new().route("/load/{name}", get(handle_load_model));
}

async fn handle_load_model(
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

    // for whatever reason returns a vector
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

    let checkpoint = CheckPoint::new(name);
    let res = checkpoint.load(profile.trainer.model.clone(), &device);

    drop(profiles);

    match res {
        Ok(loaded_model) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "message": format!("Model loaded successfully"),
                    "model": format!("{:?}", loaded_model), // Sending it back as an example
                })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": format!("Failed to load model: {}", e)})),
            );
        }
    }
}
