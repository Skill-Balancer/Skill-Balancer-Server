use burn::tensor::backend::AutodiffBackend;
use serde::{Deserialize, Serialize};

use crate::models::ppo::PPOTrainer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: usize,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    // TODO: add more parameters to allow more developer control.
    // pub trainer: PPOTrainer<B>,
}
