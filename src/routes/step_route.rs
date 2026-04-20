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
    pub game_state: Vec<ElemType>,
    pub prev_reward: ElemType,
    pub done: bool,
}

pub fn step_route() -> Router<AppState> {
    Router::new().route("/step", post(create_transition))
}

async fn create_transition(
    State(state): State<AppState>,
    Json(payload): Json<StepParam>,
) -> impl IntoResponse {
    let mut profile = state.profile.lock().await;

    let profile = match profile.as_mut() {
        Some(val) => val,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("did not find profile with name {}", payload.name)
                })),
            );
        }
    };

    let game = GameEnv {
        state: GameState::from(payload.game_state),
        reward: payload.prev_reward,
        state_size: profile.state_size,
    };

    if game.state.len() != profile.state_size {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "invalid state size",
                "expected": profile.state_size,
                "received": game.state.len()
            })),
        );
    }

    let action = match profile.trainer.step(&game, payload.done, state.metrics_tx.clone()) {
        Ok(val) => val,
        Err(val) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": format!("{}", val),
                })),
            );
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "message": format!("Stepped successfully"),
            "action": action.actions,
        })),
    )
}
