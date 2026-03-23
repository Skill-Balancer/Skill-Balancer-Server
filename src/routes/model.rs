use crate::{AppState, models::ppo::PpoTrainer, storage::model::CheckPoint};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use burn::backend::{Autodiff, NdArray, ndarray::NdArrayDevice};
use burn_rl::base::ElemType;
use serde_json::json;
pub fn save_model_route() -> Router<AppState> {
    return Router::new().route("/save", get(handle_save_model));
}

async fn handle_save_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a new one
    let name = "model".to_string();
    let checkpoint = CheckPoint::new(name.clone());

    let url = checkpoint.save(model);
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "message": format!("Model saved"),
            "url": url,
        })),
    )
}

pub fn load_model_route() -> Router<AppState> {
    return Router::new().route("/load", get(handle_load_model));
}

async fn handle_load_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a new one
    let name = "model".to_string();
    let device = NdArrayDevice::default();
    let checkpoint = CheckPoint::new(name.clone());
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
    return Router::new().route("/export/{name}", get(handle_export_model));
}

async fn handle_export_model(Path(name): Path<String>) -> impl IntoResponse {
    let checkpoint = CheckPoint::new(name.clone());
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a new one
    let device = NdArrayDevice::default();
    let res = checkpoint.export(model, &device);
    match res {
        Ok(url) => (
            StatusCode::OK,
            Json(json!({
                "message": format!("Model exported"),
                "url": url,
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Failed to export model: {}", e)})),
        ),
    }
}
