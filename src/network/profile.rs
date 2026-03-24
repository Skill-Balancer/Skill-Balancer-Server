use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, Wgpu};
use burn_rl::base::ElemType;

type Backend = Autodiff<Wgpu<ElemType>>;

pub struct Profile {
    pub id: usize,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    // TODO: add more parameters to allow more developer control.
    pub trainer: PPOTrainer<Backend>,
}
