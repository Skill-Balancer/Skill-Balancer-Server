use crate::entities::config::Model;
use crate::models::ppo::PPOTrainer;
use burn::backend::{Autodiff, Wgpu};
use burn_rl::{agent::PPOTrainingConfig, base::ElemType};

type Backend = Autodiff<Wgpu<ElemType>>;

pub struct Profile {
    pub name: String,
    #[allow(unused)]
    pub description: Option<String>,
    pub state_size: usize,
    pub train_every: usize,
    pub trainer: PPOTrainer<Backend>,
}

impl From<Model> for Profile {
    fn from(config: Model) -> Self {
        let trainer_config = PPOTrainingConfig::from(config.clone());
        let state_size = config.state.0.len();
        let train_every = config.train_every as usize;
        Self {
            name: config.name,
            description: config.description,
            state_size,
            train_every,
            trainer: PPOTrainer::<Backend>::new(trainer_config,state_size, config.actions.length() * 2 + 1, train_every),
        }
    }
}
