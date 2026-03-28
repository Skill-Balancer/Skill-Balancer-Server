use crate::{AppState, storage::model::list_exports};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

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
