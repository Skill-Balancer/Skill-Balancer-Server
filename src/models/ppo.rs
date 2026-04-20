use burn::{
    Tensor,
    module::Module,
    nn::{Initializer, Linear, LinearConfig},
    optim::{AdamW, AdamWConfig, adaptor::OptimizerAdaptor},
    prelude::Backend,
    tensor::{
        activation::{relu, softmax},
        backend::AutodiffBackend,
    },
};
use burn_rl::{
    agent::{AveragedMetrics, PPO, PPOModel, PPOOutput, PPOTrainOutput, PPOTrainingConfig},
    base::{Memory, Model},
};
use ringbuffer::RingBuffer;

use crate::{
    env::{print_steps, print_training},
    models::{action::GameAction, environment::GameEnv, metrics::Metrics, state::GameState},
};

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
const DENSE_SIZE: usize = 128;

const MEMORY_SIZE: usize = 512;

pub struct PPOTrainer<B: AutodiffBackend> {
    pub model: Net<B>,
    pub optimizer: OptimizerAdaptor<AdamW, Net<B>, B>,
    pub memory: Memory<GameEnv, B, MEMORY_SIZE>,
    pub config: PPOTrainingConfig,
    pub steps: usize,
    last_state: Option<GameState>,
    action: Option<GameAction>,
    pub train_every: usize,
}

impl<B: AutodiffBackend> PPOTrainer<B> {
    pub fn new(
        config: PPOTrainingConfig,
        input_size: usize,
        actions_amount: usize,
        train_every: usize,
    ) -> Self {
        Self {
            model: Net::new(input_size, DENSE_SIZE, actions_amount),
            optimizer: AdamWConfig::new()
                .with_grad_clipping(config.clip_grad.clone())
                .init(),
            memory: Memory::default(),
            config,
            steps: 0,
            last_state: None,
            action: None,
            train_every,
        }
    }

    pub fn step(&mut self, env: &GameEnv, done: bool) -> Result<&GameAction, String> {
        if let Some(last_state) = self.last_state.clone()
            && let Some(action) = &self.action
        {
            let current_state = &env.state;
            let reward = env.reward;
            self.memory.push(
                last_state,
                current_state.clone(),
                action.clone(),
                reward,
                done,
            );
            self.steps += 1;
            if print_steps() {
                println!(
                    "step: {}, reward: {}, memory size: {}",
                    self.steps,
                    reward,
                    self.memory.len(),
                );
                if done {
                    println!("Episode finished!");
                }
            }

            if self.train_every <= MEMORY_SIZE && self.steps.is_multiple_of(self.train_every) {
                let mean_reward = self.mean_reward();
                if let Some(m) = self.train() {
                    let metrics = Metrics {
                        step: self.steps,
                        mean_reward,
                        policy_loss: m.policy_loss,
                        value_loss: m.value_loss,
                        entropy: m.entropy,
                        clip_fraction: m.clip_fraction,
                    };
                    if print_training() {
                        println!("Metrics at step {}: {:?}", self.steps, metrics);
                    }
                }
            }
        }
        self.last_state = Some(env.state.clone());
        self.action = PPO::<GameEnv, B, Net<B>>::react_with_model(&env.state, &self.model);
        match &self.action {
            Some(val) => Ok(val),
            None => Err("something went wrong bucko".to_string()),
        }
    }

    pub fn train(&mut self) -> Option<AveragedMetrics> {
        if self.memory.len() == 0 || self.memory.len() < self.config.batch_size {
            return None;
        }

        let PPOTrainOutput { model, metrics } = PPO::<GameEnv, B, Net<B>>::train(
            self.model.clone(),
            &self.memory,
            &mut self.optimizer,
            &self.config,
        );
        self.model = model;
        self.memory.clear();
        let averaged = metrics.averaged();
        Some(averaged)
    }
    fn mean_reward(&self) -> f32 {
        let rewards = self.memory.rewards();
        if rewards.is_empty() {
            return 0.0;
        }
        rewards.iter().sum::<f32>() / rewards.len() as f32
    }
}
