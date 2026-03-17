mod profile;

use axum::{Router, response::Html, routing::{get,post}, Json, Error, http::StatusCode, response::{IntoResponse, Response}, extract::{Path, State}};
use dotenv::dotenv;
use std::{env,collections::HashMap, sync::Arc};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use crate::profile::Profile;
use tokio::sync::{Mutex, RwLock};

//importing routes and files.
mod config;
mod routes;

// importing models
mod models;

#[derive(Debug, Serialize, Deserialize)]
struct Transition {
    profile_id: String,
    step: i64,
    state: Value,
    action: Value,
    reward: f64,
    next_state: Value,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateProfileRequest {
    profile_id: String,
    name: String,
    game_id: String,
    version: String,
    description: Option<String>,
    environment: Value,
    states: Value,
    actions: Value,
    reward: Value,
    training: Value,
    output: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateSessionRequest {
    profile_id: String,
    game_id: String,
    player_id: Option<String>,
    opponent_type: String,
    metadata: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransitionItem {
    step: i64,
    state: Value,
    action: Value,
    reward: f64,
    next_state: Value,
    done: bool,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTransitionsRequest {
    transitions: Vec<TransitionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecommendationRequest {
    profile_id: String,
    session_id: Option<String>,
    state: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecommendationResponse {
    recommendation: Value,
}
#[derive(Clone)]
struct AppState {
    profiles: Arc<RwLock<HashMap<String, Profile>>>,
    transitions: Arc<Mutex<Vec<Transition>>>,
}

struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.message
        }));
        (self.status, body).into_response()
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState{
        profiles: Arc::new(RwLock::new(HashMap::new())),
        transitions: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::<()>::new()
        .merge(routes::root::get_root())
        .merge(routes::config_route::config_route())
        .merge(routes::step_route::step_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

  async fn create_profile(
      State(state): State<AppState>,
      Json(payload): Json<CreateProfileRequest>,
  ) -> ApiResult<Profile> {
      let mut profiles = state.profiles.write().await;

      if profiles.contains_key(&payload.profile_id) {
          return Err(ApiError::new(
              StatusCode::BAD_REQUEST,
              "Profile with that id already exists",
          ));
      }

      let profile = Profile {
          profile_id: payload.profile_id,
          name: payload.name,
          game_id: payload.game_id,
          version: payload.version,
          description: payload.description,
          environment: payload.environment,
          states: payload.states,
          actions: payload.actions,
          reward: payload.reward,
          training: payload.training,
          output: payload.output,
      };
      profiles.insert(profile.profile_id.clone(), profile.clone());
      Ok(Json(profile))
  }
    async fn list_profiles(
        State(state): State<AppState>,
    ) -> ApiResult<Vec<Profile>> {
        let profiles = state.profiles.read().await;
        let values = profiles.values().cloned().collect::<Vec<_>>();
        Ok(Json(values))
    }

    async fn create_transition(
        State(state): State<AppState>,
        Json(payload): Json<CreateTransitionRequest>,
    ) -> ApiResult<CreateTransitionResponse> {
        let profiles = state.profiles.read().await;

        if !profiles.contains_key(&payload.profile_id) {
            return Err(ApiError::new(
                StatusCode::BAD_REQUEST,
                "profile_id does not exist",
            ));
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
}
