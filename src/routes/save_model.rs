use crate::{
    AppState,
    models::ppo::PpoTrainer,
    storage::save_model::{load_model, save_model},
};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use burn::backend::{Autodiff, NdArray, ndarray::NdArrayDevice};
use burn_rl::base::ElemType;
pub fn save_model_route() -> Router<AppState> {
    return Router::new().route("/save", get(handle_save_model));
}

async fn handle_save_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model;
    let name = "model".to_string();
    let path = save_model(model, &name);
    (
        StatusCode::NOT_IMPLEMENTED,
        format!("Model saved at path: {}", path),
    )
}

pub fn load_model_route() -> Router<AppState> {
    return Router::new().route("/load", get(handle_load_model));
}

async fn handle_load_model() -> impl IntoResponse {
    type Back = Autodiff<NdArray<ElemType>>;
    let model = PpoTrainer::<Back>::new().model;
    let name = "model".to_string();
    let device = NdArrayDevice::default();
    let model = load_model(model, &name, &device);
    (
        StatusCode::NOT_IMPLEMENTED,
        format!("Model loaded: {}", model),
    )
}
