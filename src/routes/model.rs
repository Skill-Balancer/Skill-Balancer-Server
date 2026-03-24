use crate::{
    AppState,
    models::ppo::PPOTrainer,
    storage::model::{CheckPoint, list_exports, list_saves},
};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use burn::backend::{Autodiff, NdArray, ndarray::NdArrayDevice};
use burn_rl::{agent::PPOTrainingConfig, base::ElemType};
use serde_json::json;

pub fn save_model_route() -> Router<AppState> {
    return Router::new().route("/save/{model_id}", get(handle_save_model));
}

async fn handle_save_model(Path(model_id): Path<String>) -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let config = PPOTrainingConfig::default();
    let model = PPOTrainer::<Back>::new(config).model; // TODO: Use the actual model instead of creating a new one
    let checkpoint = CheckPoint::new(model_id.clone());

    checkpoint.save(model);
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "message": format!("Model saved"),
            "url": checkpoint.to_url(),
        })),
    )
}

pub fn load_model_route() -> Router<AppState> {
    return Router::new().route("/load/{model_id}", get(handle_load_model));
}

async fn handle_load_model(Path(model_id): Path<String>) -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let config = PPOTrainingConfig::default();
    let model = PPOTrainer::<Back>::new(config).model; // TODO: Use the actual model instead of creating a new one
    let device = NdArrayDevice::default();
    let checkpoint = CheckPoint::new(model_id);
    let res = checkpoint.load(model, &device);

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
    return Router::new().route("/export/{model_id}", get(handle_export_model));
}

async fn handle_export_model(Path(model_id): Path<String>) -> impl IntoResponse {
    let checkpoint = CheckPoint::new(model_id.clone());
    type Back = Autodiff<NdArray<ElemType>>;
    let config = PPOTrainingConfig::default();
    let model = PPOTrainer::<Back>::new(config).model; // TODO: Use the actual model instead of creating a new one
    let device = NdArrayDevice::default();
    let res = checkpoint.export(model, &device);
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
