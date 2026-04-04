use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Serialize, Deserialize)]
struct ProfilesJSON {
    name: String,
    description: Option<String>,
}

pub fn all_config_route() -> Router<AppState> {
    Router::new().route("/config/all", get(list_profiles))
}

async fn list_profiles(State(state): State<AppState>) -> impl IntoResponse {
    let profiles = state.profiles.lock().await;
    let profiles = profiles.iter();

    let mut values = vec![];
    for profile in profiles {
        values.push(ProfilesJSON {
            name: profile.name.clone(),
            description: profile.description.clone(),
        });
    }

    (StatusCode::OK, Json(json!({ "profiles": values })))
}
