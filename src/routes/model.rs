use crate::{
    AppState,
    env::models_dir,
    models::ppo::PpoTrainer,
    storage::model::{load_model, save_model},
};
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use burn::backend::{Autodiff, NdArray, ndarray::NdArrayDevice};
use burn_rl::base::ElemType;
use burn_store::{ModuleSnapshot, SafetensorsStore};
use serde_json::json;
pub fn save_model_route() -> Router<AppState> {
    return Router::new().route("/save", get(handle_save_model));
}

async fn handle_save_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a
    // new one
    let name = "model".to_string();
    save_model(model, &name);
    // Send url path back in json
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "message": format!("Model saved"),
            "url": format!("/models/{}.mpk", name),
        })),
    )
}

pub fn load_model_route() -> Router<AppState> {
    return Router::new().route("/load", get(handle_load_model));
}

async fn handle_load_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a
    let name = "model".to_string(); // new one
    let device = NdArrayDevice::default();
    let model = load_model(model, &name, &device);
    (
        StatusCode::NOT_IMPLEMENTED,
        format!("Model loaded: {}", model), // Sending it back as an example
    )
}

pub fn export_model_route() -> Router<AppState> {
    return Router::new().route("/export/{name}", get(handle_export_model));
}

async fn handle_export_model(Path(name): Path<String>) -> impl IntoResponse {
    let mut store = SafetensorsStore::from_file(format!("{}/{}.safetensors", models_dir(), name))
        .overwrite(true);
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model; // TODO: Use the actual model instead of creating a
    let res = model.save_into(&mut store);
    match res {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message": format!("Model exported"),
                "url": format!("/models/{}.safetensors", name),
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": format!("Failed to export model: {}", e)})),
        ),
    }
}
