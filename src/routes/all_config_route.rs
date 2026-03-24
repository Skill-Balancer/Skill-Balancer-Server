use crate::AppState;
use crate::network::api_error::ApiError;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct ProfilesJSON {
    id: usize,
    name: String,
    version: String,
    description: Option<String>,
}

pub fn all_config_route() -> Router<AppState> {
    Router::new().route("/config/all", get(list_profiles))
}

async fn list_profiles(State(state): State<AppState>) -> Result<Json<Vec<ProfilesJSON>>, ApiError> {
    let profiles = state.profiles.lock().await;
    let profiles_iter = profiles.iter();
    let mut values = vec![];
    for profile in profiles_iter {
        values.push(ProfilesJSON {
            id: profile.id,
            name: profile.name.clone(),
            version: profile.version.clone(),
            description: profile.description.clone(),
        });
    }
    Ok(Json(values))
}
