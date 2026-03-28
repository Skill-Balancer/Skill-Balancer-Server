use crate::{
    AppState,
    storage::model::{CheckPoint, list_exports, list_saves},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use burn::module::Module;
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

pub fn list_saves_route() -> Router<AppState> {
    return Router::new().route("/save", get(handle_saves_model));
}

async fn handle_saves_model() -> impl IntoResponse {
    let saves = list_saves();
    let saves_info: Vec<_> = saves
        .iter()
        .map(|save| {
            json!({
                "model_id": save.model_id,
                "url": save.to_url(),
            })
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "message": format!("List of saves"),
            "saves": saves_info,
        })),
    )
}

pub fn list_exports_route() -> Router<AppState> {
    return Router::new().route("/export", get(handle_exports_model));
}

async fn handle_exports_model() -> impl IntoResponse {
    let exports = list_exports();
    let exports_info: Vec<_> = exports
        .iter()
        .map(|save| {
            json!({
                "model_id": save.model_id,
                "url": save.to_export_url(),
            })
        })
        .collect();
    (
        StatusCode::OK,
        Json(json!({
            "message": format!("List of exports"),
            "saves": exports_info,
        })),
    )
}
