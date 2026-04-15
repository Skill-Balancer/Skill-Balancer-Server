use crate::base::environment::Environment;
use burn::tensor::backend::Backend;
use burn::tensor::{BasicOps, Tensor, TensorKind};
use rand::Rng;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer};
use std::marker::PhantomData;

pub type MemoryIndices = Vec<usize>;

pub fn sample_indices(indices: MemoryIndices, size: usize) -> MemoryIndices {
    let mut rng = rand::thread_rng();
    let mut sample = Vec::<usize>::new();
    for _ in 0..size {
        unsafe {
            let index = rng.gen_range(0..indices.len());
            sample.push(*indices.get_unchecked(index));
        }
    }

    sample
}

pub fn get_batch<B: Backend, const CAP: usize, T, K: TensorKind<B> + BasicOps<B>>(
    data: &ConstGenericRingBuffer<T, CAP>,
    indices: &MemoryIndices,
    converter: impl Fn(&T) -> Tensor<B, 1, K>,
) -> Tensor<B, 2, K> {
    Tensor::cat(
        indices
            .iter()
            .filter_map(|i| data.get(*i))
            .map(converter)
            .collect::<Vec<_>>(),
        0,
    )
    .reshape([indices.len() as i32, -1])
}

pub struct Memory<E: Environment, B: Backend, const CAP: usize> {
    state: ConstGenericRingBuffer<E::StateType, CAP>,
    next_state: ConstGenericRingBuffer<E::StateType, CAP>,
    action: ConstGenericRingBuffer<E::ActionType, CAP>,
    reward: ConstGenericRingBuffer<E::RewardType, CAP>,
    done: ConstGenericRingBuffer<bool, CAP>,
    environment: PhantomData<E>,
    backend: PhantomData<B>,
}

impl<E: Environment, B: Backend, const CAP: usize> Default for Memory<E, B, CAP> {
    fn default() -> Self {
        Self {
            state: ConstGenericRingBuffer::new(),
            next_state: ConstGenericRingBuffer::new(),
            action: ConstGenericRingBuffer::new(),
            reward: ConstGenericRingBuffer::new(),
            done: ConstGenericRingBuffer::new(),
            environment: PhantomData,
            backend: PhantomData,
        }
    }
}

impl<E: Environment, B: Backend, const CAP: usize> Memory<E, B, CAP> {
    pub fn push(
        &mut self,
        state: E::StateType,
        next_state: E::StateType,
        action: E::ActionType,
        reward: E::RewardType,
        done: bool,
    ) {
        self.state.push(state);
        self.next_state.push(next_state);
        self.action.push(action);
        self.reward.push(reward);
        self.done.push(done);
    }

    pub fn states(&self) -> &ConstGenericRingBuffer<E::StateType, CAP> {
        &self.state
    }

    pub fn next_states(&self) -> &ConstGenericRingBuffer<E::StateType, CAP> {
        &self.next_state
    }

    pub fn actions(&self) -> &ConstGenericRingBuffer<E::ActionType, CAP> {
        &self.action
    }

    pub fn rewards(&self) -> &ConstGenericRingBuffer<E::RewardType, CAP> {
        &self.reward
    }

    pub fn dones(&self) -> &ConstGenericRingBuffer<bool, CAP> {
        &self.done
    }

    pub fn len(&self) -> usize {
        self.state.len()
    }

    pub fn is_empty(&self) -> bool {
        self.state.is_empty()
    }

    pub fn clear(&mut self) {
        self.state.clear();
        self.next_state.clear();
        self.action.clear();
        self.reward.clear();
        self.done.clear();
    }
}
