use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
    pub profile_id: String,
    pub state: Value,
    pub reward: f64,
}
