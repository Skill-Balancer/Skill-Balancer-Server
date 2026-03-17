use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransitionResponse {
    pub profile_id: String,
    pub step: i64,
    pub message: String,
}