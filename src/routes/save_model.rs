use crate::AppState;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
pub fn save_model() -> Router<AppState> {
    return Router::new().route("/save", get(handle_save_model));
}

async fn handle_save_model() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        "Model saving is not implemented yet.",
    )
}
