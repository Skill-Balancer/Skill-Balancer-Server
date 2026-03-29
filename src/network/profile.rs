use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, NdArray};
use burn_rl::base::ElemType;

type Backend = Autodiff<NdArray<ElemType>>;

pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub trainer: PPOTrainer<Backend>,
    // TODO: add more parameters to allow more developer control.
}
