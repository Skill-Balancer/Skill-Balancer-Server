use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransitionResponse {
    pub profile_id: String,
    pub message: String,
}
