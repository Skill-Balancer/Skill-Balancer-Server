use burn::{
    Tensor,
    module::{AutodiffModule, Module},
    nn::{Initializer, Linear, LinearConfig},
    optim::{AdamW, AdamWConfig, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{
        activation::{relu, softmax},
        backend::AutodiffBackend,
    },
};
use burn_rl::{
    agent::{PPO, PPOModel, PPOOutput, PPOTrainingConfig},
    base::{Environment, Memory, Model},
};
use tokio::runtime::TryCurrentError;

use crate::models::{action::GameAction, environment::GameEnv, state::GameState};

#[derive(Module, Debug)]
pub struct Net<B: Backend> {
    linear: Linear<B>,
    linear_actor: Linear<B>,
    linear_critic: Linear<B>,
}

impl<B: Backend> PPOModel<B> for Net<B> {}

impl<B: Backend> Net<B> {
    pub fn new(input_size: usize, dense_size: usize, output_size: usize) -> Self {
        let initializer = Initializer::XavierUniform { gain: 1.0 };
        Self {
            linear: LinearConfig::new(input_size, dense_size)
                .with_initializer(initializer.clone())
                .init(&Default::default()),
            linear_actor: LinearConfig::new(dense_size, output_size)
                .with_initializer(initializer.clone())
                .init(&Default::default()),
            linear_critic: LinearConfig::new(dense_size, 1)
                .with_initializer(initializer)
                .init(&Default::default()),
        }
    }
}

impl<B: Backend> Model<B, Tensor<B, 2>, PPOOutput<B>, Tensor<B, 2>> for Net<B> {
    fn forward(&self, input: Tensor<B, 2>) -> PPOOutput<B> {
        let layer_0_output = relu(self.linear.forward(input));
        let policies = softmax(self.linear_actor.forward(layer_0_output.clone()), 1);
        let values = self.linear_critic.forward(layer_0_output);

        PPOOutput::<B>::new(policies, values)
    }

    fn infer(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let layer_0_output = relu(self.linear.forward(input));
        softmax(self.linear_actor.forward(layer_0_output.clone()), 1)
    }
}
const INPUT_SIZE: usize = 50; // TODO: Make configuable
const DENSE_SIZE: usize = 128;
const OUTPUT_SIZE: usize = 1; // TODO: Make configuable

const MEMORY_SIZE: usize = 512;

pub const TRAIN_EVERY: usize = MEMORY_SIZE;

pub struct PpoTrainer<B: AutodiffBackend> {
    pub model: Net<B>,
    pub optimizer: OptimizerAdaptor<AdamW, Net<B>, B>,
    pub memory: Memory<GameEnv, B, MEMORY_SIZE>,
    pub config: PPOTrainingConfig,
    pub steps: usize,
    last_state: Option<GameState>,
    action: Option<GameAction>,
}

impl<B: AutodiffBackend> PpoTrainer<B> {
    pub fn new() -> Self {
        let config = PPOTrainingConfig::default();
        Self {
            model: Net::new(INPUT_SIZE, DENSE_SIZE, OUTPUT_SIZE),
            optimizer: AdamWConfig::new()
                .with_grad_clipping(config.clip_grad.clone())
                .init(),
            memory: Memory::default(),
            config: config,
            steps: 0,
            last_state: None,
            action: None,
        }
    }

    pub fn step(&mut self, env: &GameEnv) {
        if let Some(last_state) = self.last_state
            && let Some(action) = self.action
        {
            let current_state = &env.state;
            let reward = env.reward;
            self.memory.push(
                last_state,
                current_state.clone(),
                action.clone(),
                reward,
                false,
            );
            self.steps += 1;

            if self.steps % TRAIN_EVERY == 0 {
                // self.train
                todo!();
            }
        }
        self.last_state = Some(env.state.clone());
        self.action = PPO::<GameEnv, B, Net<B>>::react_with_model(&env.state, &self.model);
    }

    fn train(&self) {
        todo!(); // Use PPO<>::train function (See example)
    }
}
