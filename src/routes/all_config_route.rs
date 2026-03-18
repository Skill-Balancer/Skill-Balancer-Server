use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use crate::{AppState};
use crate::models::profile::Profile;
use crate::models::apiError::ApiError;

pub fn all_config_route() -> Router<AppState> {
    Router::new().route("/config/all", get(list_profiles))
}


async fn list_profiles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Profile>>, ApiError> {
    let profiles = state.profiles.read().await;
    let values = profiles.values().cloned().collect::<Vec<_>>();
    Ok(Json(values))
}