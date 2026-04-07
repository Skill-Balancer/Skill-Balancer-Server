use crate::AppState;
use crate::entities::config;
use crate::network::profile::Profile;
use crate::storage::model::delete_config_files;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router, http::StatusCode, routing::post};
use burn::grad_clipping::GradientClippingConfig;
use burn_rl::agent::PPOTrainingConfig;
use burn_rl::base::ElemType;
use sea_orm::ActiveValue::Set;
use sea_orm::TryIntoModel;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigParams {
    name: String,
    description: Option<String>,

    state_size: u32,
    action_size: u32,
    train_every: Option<u32>,

    // PPO config (completely optional and will be handled if none)
    gamma: Option<ElemType>,
    lambda: Option<ElemType>,
    epsilon_clip: Option<ElemType>,
    critic_weight: Option<ElemType>,
    entropy_weight: Option<ElemType>,
    learning_rate: Option<ElemType>,
    epochs: Option<u32>,
    batch_size: Option<u32>,
    clip_grad: Option<f32>,

    allow_overwrite: Option<bool>,
}

pub fn config_route() -> Router<AppState> {
    Router::new().route("/config", post(create_profile))
}

async fn create_profile(
    State(state): State<AppState>,
    Json(payload): Json<ConfigParams>,
) -> impl IntoResponse {
    let request_active_model = get_active_model_from_config(&payload);
    let request_model = match request_active_model.clone().try_into_model() {
        Ok(model) => model,
        Err(e) => {
            eprintln!("Model conversion error: {}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid config parameters."})),
            );
        }
    };

    let allow_overwrite = payload.allow_overwrite.unwrap_or(false);

    match (
        state.db.get_config(&request_model.name).await,
        allow_overwrite,
    ) {
        (Ok(Some(db_model)), false) => try_cmp_configs(&state, &db_model, &request_model).await,
        (Ok(Some(_)), true) => try_update_config(&state, request_active_model).await,
        (Ok(None), _) => try_insert_config(&state, request_active_model).await,
        (Err(e), _) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to query database for existing config."})),
            )
        }
    }
}

async fn update_profile(state: &AppState, config: config::Model) {
    let mut profile = state.profile.lock().await;
    let new_profile = Profile::from(config);
    let profile_name = new_profile.name.clone();
    *profile = Some(new_profile);
    println!("Profile '{}' is now active.", profile_name);
}

async fn try_cmp_configs(
    state: &AppState,
    db_model: &config::Model,
    request_model: &config::Model,
) -> (StatusCode, Json<serde_json::Value>) {
    if cmp_configs(&db_model, &request_model) {
        update_profile(state, db_model.clone()).await;
        (
            StatusCode::OK,
            Json(json!({"message": "Using existing profile."})),
        )
    } else {
        (
            StatusCode::CONFLICT,
            Json(json!({"message": "Config already exists with different configuration."})),
        )
    }
}

async fn try_insert_config(
    state: &AppState,
    config: config::ActiveModel,
) -> (StatusCode, Json<serde_json::Value>) {
    let result = state.db.insert_config(config).await;
    match result {
        Ok(config) => {
            update_profile(state, config).await;
            (
                StatusCode::OK,
                Json(json!({"message": "Config created and saved to database successfully."})),
            )
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to save config to database."})),
            )
        }
    }
}

async fn try_update_config(
    state: &AppState,
    config: config::ActiveModel,
) -> (StatusCode, Json<serde_json::Value>) {
    let result = state.db.update_config(config).await;
    match result {
        Ok(new_config) => {
            delete_config_files(&new_config.name);
            update_profile(state, new_config).await;
            (
                StatusCode::OK,
                Json(json!({"message": "Config has been overwritten successfully."})),
            )
        }
        Err(e) => {
            eprintln!("Database error during update: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Failed to update config in database."})),
            )
        }
    }
}

fn cmp_configs(db_conf: &config::Model, request: &config::Model) -> bool {
    db_conf.gamma == request.gamma
        && db_conf.lambda == request.lambda
        && db_conf.epsilon_clip == request.epsilon_clip
        && db_conf.critic_weight == request.critic_weight
        && db_conf.entropy_weight == request.entropy_weight
        && db_conf.learning_rate == request.learning_rate
        && db_conf.epochs == request.epochs
        && db_conf.batch_size == request.batch_size
        && db_conf.clip_grad == request.clip_grad
        && db_conf.state_size == request.state_size
        && db_conf.action_size == request.action_size
        && db_conf.train_every == request.train_every
}

fn get_active_model_from_config(config: &ConfigParams) -> crate::entities::config::ActiveModel {
    let defaults = PPOTrainingConfig::default();

    crate::entities::config::ActiveModel {
        name: Set(config.name.clone()),
        description: Set(config.description.clone()),
        state_size: Set(config.state_size),
        action_size: Set(config.action_size),
        train_every: Set(config.train_every.unwrap_or(300)),
        gamma: Set(config.gamma.unwrap_or(defaults.gamma)),
        lambda: Set(config.lambda.unwrap_or(defaults.lambda)),
        epsilon_clip: Set(config.epsilon_clip.unwrap_or(defaults.epsilon_clip)),
        critic_weight: Set(config.critic_weight.unwrap_or(defaults.critic_weight)),
        entropy_weight: Set(config.entropy_weight.unwrap_or(defaults.entropy_weight)),
        learning_rate: Set(config.learning_rate.unwrap_or(defaults.learning_rate)),
        epochs: Set(config.epochs.unwrap_or(defaults.epochs as u32)),
        batch_size: Set(config.batch_size.unwrap_or(defaults.batch_size as u32)),
        clip_grad: Set(config.clip_grad.unwrap_or(0.5)),
    }
}

impl From<config::Model> for PPOTrainingConfig {
    fn from(config: config::Model) -> Self {
        PPOTrainingConfig {
            gamma: config.gamma,
            lambda: config.lambda,
            epsilon_clip: config.epsilon_clip,
            critic_weight: config.critic_weight,
            entropy_weight: config.entropy_weight,
            learning_rate: config.learning_rate,
            epochs: config.epochs as usize,
            batch_size: config.batch_size as usize,
            clip_grad: Some(GradientClippingConfig::Value(config.clip_grad)),
        }
    }
}
