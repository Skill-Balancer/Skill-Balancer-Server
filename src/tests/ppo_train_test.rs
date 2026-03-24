use burn::backend::{Autodiff, NdArray};
use burn::tensor::Tensor;

use crate::models::action::GameAction;
use crate::models::ppo::PPOTrainer;
use crate::models::state::GameState;
use burn_rl::agent::PPOTrainingConfig;
use burn_rl::base::Model;
use burn_rl::base::State;

type TestBackend = Autodiff<NdArray>;

fn make_state(value: f32) -> GameState {
    GameState {
        data: [value, value, value, value],
    }
}

fn fill_memory_for_training(trainer: &mut PPOTrainer<TestBackend>, count: usize) {
    for i in 0..count {
        let state = make_state(0.1 + i as f32 * 0.001);
        let next_state = make_state(0.2 + i as f32 * 0.001);

        let action = match i % 5 {
            0 => GameAction::NoChange,
            1 => GameAction::PlayerOneAttInc,
            2 => GameAction::PlayerOneAttDec,
            3 => GameAction::PlayerTwoAttInc,
            _ => GameAction::PlayerTwoAttDec,
        };

        trainer.memory.push(state, next_state, action, 1.0, false);
    }
}

#[test]
fn train_resets_memory() {
    let mut trainer = PPOTrainer::<TestBackend>::new(PPOTrainingConfig::default());

    let batch_size = trainer.config.batch_size;
    fill_memory_for_training(&mut trainer, batch_size);

    assert!(trainer.memory.len() >= batch_size);

    trainer.train();

    assert_eq!(trainer.memory.len(), 0);
}
fn flatten_tensor_2d(tensor: burn::Tensor<TestBackend, 2>) -> Vec<f32> {
    let data = tensor.into_data();

    data.to_vec::<f32>().expect("failed to flatten tensor")
}
#[test]
fn train_changes_model_output() {
    let mut trainer = PPOTrainer::<TestBackend>::new(PPOTrainingConfig::default());

    let test_state = make_state(0.5);
    let input = test_state.to_tensor::<TestBackend>().unsqueeze();

    let before = trainer.model.forward(input.clone());
    let before_policies = flatten_tensor_2d(before.policies);
    let before_values = flatten_tensor_2d(before.values);

    let batch_size = trainer.config.batch_size;
    fill_memory_for_training(&mut trainer, batch_size);

    trainer.train();

    let after = trainer.model.forward(input);
    let after_policies = flatten_tensor_2d(after.policies);
    let after_values = flatten_tensor_2d(after.values);

    let policies_changed = before_policies != after_policies;
    let values_changed = before_values != after_values;

    assert!(
        policies_changed || values_changed,
        "Model output did not change after training"
    );
}
