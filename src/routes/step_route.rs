use axum::{Router, http::StatusCode, routing::post, Json};
use axum::extract::State;
use crate::{AppState};
use crate::models::createTransitionResponse::CreateTransitionResponse;
use crate::models::transition::Transition;
use crate::models::apiError::ApiError;

pub fn step_route() -> Router<AppState> {
     Router::new().route("/step", post(create_transition))
}

async fn create_transition(
    State(state): State<AppState>,
    Json(payload): Json<Transition>,
) -> Result<Json<CreateTransitionResponse>, ApiError> {
    let profiles = state.profiles.read().await;

    if !profiles.contains_key(&payload.profile_id) {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            message: "Profile with that id does not exist".to_string(),
        });
    }
    drop(profiles);

    let transition = Transition {
        profile_id: payload.profile_id.clone(),
        state: payload.state,
        reward: payload.reward,
    };
    let response = CreateTransitionResponse {
        profile_id: transition.profile_id.clone(),
        message: "Transition stored successfully".to_string(),
        
    };
    let mut transitions = state.transitions.lock().await;
    transitions.push(transition);

    Ok(Json(response))
}
