use crate::AppState;
use crate::models::environment::GameEnv;
use crate::models::state::GameState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router, routing::post};
use burn_rl::base::ElemType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StepParam {
    pub id: usize,
    pub game_state: [ElemType; 4],
    pub reward: ElemType,
}

pub fn step_route() -> Router<AppState> {
    Router::new().route("/step", post(create_transition))
}

async fn create_transition(
    State(state): State<AppState>,
    Json(payload): Json<StepParam>,
) -> impl IntoResponse {
    let game = GameEnv {
        state: GameState::from(payload.game_state),
        reward: payload.reward,
    };

    let mut profiles = state.profiles.lock().await;

    let profile = match profiles.get_mut(payload.id) {
        Some(val) => val,
        None => return StatusCode::NOT_FOUND,
    };

    profile.trainer.step(&game);
    drop(profiles);

    StatusCode::OK
}
