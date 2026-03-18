use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub profile_id: String,
    pub name: String,
    pub game_id: String,
    pub version: String,
    pub description: Option<String>,
    pub states: Value,
    pub actions: Value,
}
