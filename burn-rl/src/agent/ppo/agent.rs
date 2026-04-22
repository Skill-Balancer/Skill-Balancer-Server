use crate::agent::ppo::model::{PPOModel, PPOOutput};
use crate::agent::PPOTrainingConfig;
use crate::base::{get_batch, sample_indices, ElemType, Environment, Memory, MemoryIndices};
use crate::utils::{
    elementwise_min, get_elem, ref_to_action_tensor, ref_to_not_done_tensor, ref_to_reward_tensor,
    ref_to_state_tensor, sample_action_from_tensor, update_parameters,
};
use burn::module::AutodiffModule;
use burn::nn::loss::{MseLoss, Reduction};
use burn::optim::Optimizer;
use burn::tensor::backend::{AutodiffBackend, Backend};
use burn::tensor::{ElementConversion, Tensor};
use std::marker::PhantomData;

pub struct PPO<E: Environment, B: Backend, M: PPOModel<B>> {
    #[allow(unused)]
    model: Option<M>,
    state: PhantomData<E::StateType>,
    action: PhantomData<E::ActionType>,
    backend: PhantomData<B>,
}

impl<E: Environment, B: Backend, M: PPOModel<B>> PPO<E, B, M> {
    pub fn new(model: M) -> Self {
        Self {
            model: Some(model),
            state: PhantomData,
            action: PhantomData,
            backend: PhantomData,
        }
    }

    pub fn react_with_model(state: &E::StateType, model: &M) -> Option<E::ActionType> {
        sample_action_from_tensor::<E::ActionType, _>(
            model.infer(ref_to_state_tensor(state).unsqueeze()),
        )
    }
}

impl<E: Environment, B: Backend, M: PPOModel<B>> Default for PPO<E, B, M> {
    fn default() -> Self {
        Self {
            model: None,
            state: PhantomData,
            action: PhantomData,
            backend: PhantomData,
        }
    }
}

impl<E: Environment, B: AutodiffBackend, M: PPOModel<B> + AutodiffModule<B>> PPO<E, B, M> {
    pub fn train<const CAP: usize>(
        mut policy_net: M,
        memory: &Memory<E, B, CAP>,
        optimizer: &mut (impl Optimizer<M, B> + Sized),
        config: &PPOTrainingConfig,
    ) -> PPOTrainOutput<M> {
        let mut epoch_metrics = EpochMetrics::default();
        let memory_indices = (0..memory.len()).collect::<MemoryIndices>();
        let PPOOutput {
            policies: mut old_polices,
            values: mut old_values,
        } = policy_net.forward(get_batch(
            memory.states(),
            &memory_indices,
            ref_to_state_tensor,
        ));
        old_polices = old_polices.detach();
        old_values = old_values.detach();

        if let Some(GAEOutput {
            expected_returns,
            advantages,
        }) = get_gae(
            old_values,
            get_batch(memory.rewards(), &memory_indices, ref_to_reward_tensor),
            get_batch(memory.dones(), &memory_indices, ref_to_not_done_tensor),
            config.gamma,
            config.lambda,
        ) {
            for _ in 0..config.epochs {
                for _ in 0..(memory.len() / config.batch_size) {
                    let sample_indices = sample_indices(memory_indices.clone(), config.batch_size);

                    let sample_indices_tensor = Tensor::from_ints(
                        sample_indices
                            .iter()
                            .map(|x| *x as i32)
                            .collect::<Vec<_>>()
                            .as_slice(),
                        &Default::default(),
                    );
                    let state_batch =
                        get_batch(memory.states(), &sample_indices, ref_to_state_tensor);
                    let action_batch =
                        get_batch(memory.actions(), &sample_indices, ref_to_action_tensor);
                    let old_policy_batch =
                        old_polices.clone().select(0, sample_indices_tensor.clone());
                    let advantage_batch =
                        advantages.clone().select(0, sample_indices_tensor.clone());
                    let expected_return_batch = expected_returns
                        .clone()
                        .select(0, sample_indices_tensor)
                        .detach();

                    let PPOOutput {
                        policies: policy_batch,
                        values: value_batch,
                    } = policy_net.forward(state_batch);

                    let ratios = policy_batch
                        .clone()
                        .div(old_policy_batch.clamp(1e-8, 1.0)) // clamp added to prevent action for being none
                        .gather(1, action_batch);
                    let clipped_ratios = ratios
                        .clone()
                        .clamp(1.0 - config.epsilon_clip, 1.0 + config.epsilon_clip);

                    let actor_loss = -elementwise_min(
                        ratios.clone() * advantage_batch.clone(),
                        clipped_ratios * advantage_batch,
                    )
                    .mean();
                    let critic_loss =
                        MseLoss.forward(expected_return_batch, value_batch, Reduction::Mean);
                    let policy_negative_entropy = -(policy_batch.clone().clamp(1e-8, 1.0).log()
                        * policy_batch) // clamp added to prevent action for being none
                        .sum_dim(1)
                        .mean();

                    // Convert losses to scalars for logging
                    let policy_loss_scalar = actor_loss.clone().into_scalar().elem::<f32>();
                    let value_loss_scalar = critic_loss.clone().into_scalar().elem::<f32>();
                    let entropy_scalar =
                        policy_negative_entropy.clone().into_scalar().elem::<f32>();

                    let clip_fraction_scalar = {
                        let lo = 1.0 - config.epsilon_clip;
                        let hi = 1.0 + config.epsilon_clip;
                        let ratio_data: Vec<f32> = ratios
                            .clone()
                            .into_data()
                            .to_vec::<f32>()
                            .unwrap_or_default();
                        let clipped = ratio_data.iter().filter(|&&r| r < lo || r > hi).count();
                        clipped as f32 / ratio_data.len().max(1) as f32
                    };

                    epoch_metrics.record(
                        policy_loss_scalar,
                        value_loss_scalar,
                        entropy_scalar,
                        clip_fraction_scalar,
                    );
                    let loss = actor_loss
                        + critic_loss.mul_scalar(config.critic_weight)
                        + policy_negative_entropy.mul_scalar(config.entropy_weight);
                    policy_net =
                        update_parameters(loss, policy_net, optimizer, config.learning_rate.into());
                }
            }
        }
        PPOTrainOutput {
            model: policy_net,
            metrics: epoch_metrics,
        }
    }

    pub fn valid(&self, model: M) -> PPO<E, B::InnerBackend, M::InnerModule>
    where
        <M as AutodiffModule<B>>::InnerModule: PPOModel<<B as AutodiffBackend>::InnerBackend>,
    {
        PPO::<E, B::InnerBackend, M::InnerModule>::new(model.valid())
    }
}

pub(crate) struct GAEOutput<B: Backend> {
    expected_returns: Tensor<B, 2>,
    advantages: Tensor<B, 2>,
}

impl<B: Backend> GAEOutput<B> {
    fn new(expected_returns: Tensor<B, 2>, advantages: Tensor<B, 2>) -> Self {
        Self {
            expected_returns,
            advantages,
        }
    }
}

pub(crate) fn get_gae<B: Backend>(
    values: Tensor<B, 2>,
    rewards: Tensor<B, 2>,
    not_dones: Tensor<B, 2>,
    gamma: ElemType,
    lambda: ElemType,
) -> Option<GAEOutput<B>> {
    let mut returns = vec![0.0 as ElemType; rewards.shape().num_elements()];
    let mut advantages = returns.clone();

    let mut running_return: ElemType = 0.0;
    let mut running_advantage: ElemType = 0.0;

    for i in (0..rewards.shape().num_elements()).rev() {
        let reward = get_elem(i, &rewards)?;
        let not_done = get_elem(i, &not_dones)?;

        running_return = reward + gamma * running_return * not_done;
        running_advantage = (reward + gamma * not_done * get_elem(i + 1, &values).unwrap_or(0.0)
            - get_elem(i, &values)?)
            + gamma * lambda * not_done * running_advantage;

        returns[i] = running_return;
        advantages[i] = running_advantage;
    }
    let adv_mean = advantages.iter().sum::<f32>() / advantages.len() as f32;
    let adv_std = (advantages
        .iter()
        .map(|a| (a - adv_mean).powi(2))
        .sum::<f32>()
        / advantages.len() as f32)
        .sqrt()
        .max(1e-8);
    let advantages: Vec<f32> = advantages
        .iter()
        .map(|a| (a - adv_mean) / adv_std)
        .collect();
    Some(GAEOutput::new(
        Tensor::<B, 1>::from_floats(returns.as_slice(), &Default::default())
            .reshape([returns.len(), 1]),
        Tensor::<B, 1>::from_floats(advantages.as_slice(), &Default::default())
            .reshape([advantages.len(), 1]),
    ))
}
pub struct PPOTrainOutput<M> {
    pub model: M,
    pub metrics: EpochMetrics,
}

#[derive(Default)]
pub struct EpochMetrics {
    pub policy_loss: f32,
    pub value_loss: f32,
    pub entropy: f32,
    pub clip_fraction: f32,
    pub updates: usize,
}

impl EpochMetrics {
    pub fn record(&mut self, policy_loss: f32, value_loss: f32, entropy: f32, clip_fraction: f32) {
        self.policy_loss += policy_loss;
        self.value_loss += value_loss;
        self.entropy += entropy;
        self.clip_fraction += clip_fraction;
        self.updates += 1;
    }

    pub fn averaged(self) -> AveragedMetrics {
        let n = self.updates.max(1) as f32;
        AveragedMetrics {
            policy_loss: self.policy_loss / n,
            value_loss: self.value_loss / n,
            entropy: self.entropy / n,
            clip_fraction: self.clip_fraction / n,
        }
    }
}

pub struct AveragedMetrics {
    pub policy_loss: f32,
    pub value_loss: f32,
    pub entropy: f32,
    pub clip_fraction: f32,
}
