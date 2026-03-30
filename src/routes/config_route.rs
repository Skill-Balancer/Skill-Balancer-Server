use crate::AppState;
use crate::models::ppo::PPOTrainer;
use crate::network::profile::Profile;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router, http::StatusCode, routing::post};
use burn::grad_clipping::GradientClippingConfig;
use burn_rl::agent::PPOTrainingConfig;
use burn_rl::base::ElemType;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigParams {
    name: String,
    description: Option<String>,

    // TODO: make more types for model, optimizer, memory and so on

    // PPO config (completely optional and will be handled if none)
    gamma: Option<ElemType>,
    lambda: Option<ElemType>,
    epsilon_clip: Option<ElemType>,
    critic_weight: Option<ElemType>,
    entropy_weight: Option<ElemType>,
    learning_rate: Option<ElemType>,
    epochs: Option<u64>,
    batch_size: Option<u64>,
    clip_grad: Option<f32>,
}

pub fn config_route() -> Router<AppState> {
    Router::new().route("/config", post(create_profile))
}

async fn create_profile(
    State(state): State<AppState>,
    Json(payload): Json<ConfigParams>,
) -> impl IntoResponse {
    let config = set_config(&payload);
    let mut profiles = state.profiles.lock().await;

    state
        .db
        .insert_config(crate::entities::config::ActiveModel {
            name: Set(payload.name.clone()),
            description: Set(Some(payload.description.clone().unwrap_or("".to_string()))),
            gamma: Set(config.gamma),
            lambda: Set(config.lambda),
            epsilon_clip: Set(config.epsilon_clip),
            critic_weight: Set(config.critic_weight),
            entropy_weight: Set(config.entropy_weight),
            learning_rate: Set(config.learning_rate),
            epochs: Set(config.epochs as u32),
            batch_size: Set(config.batch_size as u32),
            clip_grad: Set(payload.clip_grad.unwrap_or(100.0)),
        })
        .await
        .expect("Failed to insert config into database");

    let profile = Profile {
        name: payload.name,
        description: payload.description,
        trainer: PPOTrainer::new(config),
    };

    profiles.push(profile);
    (
        StatusCode::OK,
        Json(json!({
            "message": format!("Successfully created profile!")})),
    )
}

fn set_config(payload: &ConfigParams) -> PPOTrainingConfig {
    let mut config = PPOTrainingConfig::default();
    config.gamma = payload.gamma.unwrap_or_else(|| config.gamma);
    config.lambda = payload.lambda.unwrap_or_else(|| config.lambda);
    config.epsilon_clip = payload.epsilon_clip.unwrap_or_else(|| config.epsilon_clip);
    config.critic_weight = payload
        .critic_weight
        .unwrap_or_else(|| config.critic_weight);
    config.entropy_weight = payload
        .entropy_weight
        .unwrap_or_else(|| config.entropy_weight);
    config.learning_rate = payload
        .learning_rate
        .unwrap_or_else(|| config.learning_rate);
    config.epochs = payload.epochs.unwrap_or_else(|| config.epochs as u64) as usize;
    config.batch_size = payload.batch_size.unwrap_or_else(|| config.batch_size as u64) as usize;

    config.clip_grad = match payload.clip_grad {
        None => Some(GradientClippingConfig::Value(100.0)),
        Some(val) => Some(GradientClippingConfig::Value(val)),
    };

    config
}
