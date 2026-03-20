use crate::AppState;
use crate::network::api_error::ApiError;
use crate::network::profile::Profile;
use axum::extract::State;
use axum::{Json, Router, http::StatusCode, routing::post};

pub fn config_route() -> Router<AppState> {
    Router::new().route("/config", post(create_profile))
}

async fn create_profile(
    State(state): State<AppState>,
    Json(payload): Json<Profile>,
) -> Result<Json<Profile>, ApiError> {
    let mut profiles = state.profiles.write().await;

    if profiles.contains_key(&payload.profile_id) {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Profile with that id already exists".to_string(),
        });
    }

    let profile = Profile {
        profile_id: payload.profile_id,
        name: payload.name,
        game_id: payload.game_id,
        version: payload.version,
        description: payload.description,
        states: payload.states,
        actions: payload.actions,
    };
    profiles.insert(profile.profile_id.clone(), profile.clone());
    Ok(Json(profile))
}
