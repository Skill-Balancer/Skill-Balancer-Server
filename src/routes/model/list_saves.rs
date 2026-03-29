use crate::{AppState, storage::model::list_saves};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

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
