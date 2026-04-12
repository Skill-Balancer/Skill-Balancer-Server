use crate::entities::config::Model;
use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, Wgpu};
use burn_rl::base::ElemType;

type Backend = Autodiff<Wgpu<ElemType>>;

pub struct Profile {
    pub name: String,
    pub description: Option<String>,
    pub trainer: PPOTrainer<Backend>,
}

impl From<Model> for Profile {
    fn from(config: Model) -> Self {
        let trainer_config = burn_rl::agent::PPOTrainingConfig::from(config.clone());
        Self {
            name: config.name,
            description: config.description,
            trainer: PPOTrainer::<Backend>::new(trainer_config, config.actions.length()),
        }
    }
}
