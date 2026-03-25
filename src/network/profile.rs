use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, NdArray};
use burn_rl::base::ElemType;

type Backend = Autodiff<NdArray<ElemType>>;

pub struct Profile {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub trainer_index: Vec<String>,
    pub trainer: PPOTrainer<Backend>,
    // TODO: add more parameters to allow more developer control.
}

// TODO: make function that can switch between different trainers
