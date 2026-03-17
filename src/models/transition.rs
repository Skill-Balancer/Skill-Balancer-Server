use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transition {
    pub profile_id: String,
    pub step: i64,
    pub state: Value,
    pub action: Value,
    pub reward: f64,
    pub next_state: Value,
    pub done: bool,
}