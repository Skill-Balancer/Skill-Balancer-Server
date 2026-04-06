use crate::AppState;
use crate::models::environment::GameEnv;
use crate::models::state::GameState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router, routing::post};
use burn_rl::base::ElemType;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct StepParam {
    pub name: String,
    pub game_state: [ElemType; 4],
    pub prev_reward: ElemType,
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
        reward: payload.prev_reward,
    };

    match state.profile.lock().await.as_mut() {
        Some(profile) => {
            let action = profile.trainer.step(&game);

            (
                StatusCode::OK,
                Json(json!({
                    "message": format!("Stepped successfully"),
                    "action": action,
                })),
            )
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "message": format!("Could not find profile"),
                })),
            );
        }
    }
}
