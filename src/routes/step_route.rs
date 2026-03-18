use axum::{Router, http::StatusCode, routing::post, Json};
use axum::extract::State;
use crate::{ApiError, AppState};
use crate::models::createTransitionResponse::CreateTransitionResponse;
use crate::models::transition::Transition;

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
        step: payload.step,
        state: payload.state,
        action: payload.action,
        reward: payload.reward,
        next_state: payload.next_state,
        done: payload.done,
    };
    let response = CreateTransitionResponse {
        profile_id: transition.profile_id.clone(),
        step: transition.step,
        message: "Transition stored successfully".to_string(),
        
    };
    let mut transitions = state.transitions.lock().await;
    transitions.push(transition);

    Ok(Json(response))
}
