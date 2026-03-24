use crate::network::api_error::ApiError;
use crate::network::app_state::AppState;
use crate::network::transition::Transition;
use axum::extract::State;
use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateTransitionResponse {
    pub profile_id: String,
    pub message: String,
}

pub fn step_route() -> Router<AppState> {
    Router::new().route("/step", post(create_transition))
}

async fn create_transition(
    State(state): State<AppState>,
    Json(payload): Json<Transition>,
) -> Result<Json<CreateTransitionResponse>, ApiError> {
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
