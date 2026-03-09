use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub game_id: String,
    pub version: String,
    pub description: Option<String>,
    pub environment: Value,
    pub state_schema: Value,
    pub action_schema: Value,
    pub reward: Value,
    pub training: Value,
    pub output: Value,
}

