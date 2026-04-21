use crate::{AppState, storage::model::list_exports};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

pub fn list_exports_route() -> Router<AppState> {
    return Router::new().route("/export", get(handle_exports_model));
}

async fn handle_exports_model(State(state): State<AppState>) -> impl IntoResponse {
    match state.profile.lock().await.as_ref() {
        Some(profile) => {
            let exports = list_exports(profile.name.clone());
            let exports_info: Vec<_> = exports
                .iter()
                .map(|save| {
                    json!({
                        "model_id": save.id,
                        "url": save.to_export_url(),
                    })
                })
                .collect();
            (
                StatusCode::OK,
                Json(json!({
                    "message": format!("List of exports"),
                    "exports": exports_info,
                })),
            )
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Could not find default profile, this error should never print!"),
                })),
            );
        }
    }
}
