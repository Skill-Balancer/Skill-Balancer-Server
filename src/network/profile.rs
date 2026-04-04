use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, Wgpu};
use burn_rl::base::ElemType;

type Backend = Autodiff<Wgpu<ElemType>>;

pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub trainer: PPOTrainer<Backend>,
    // TODO: add more parameters to allow more developer control.
}
