use burn::tensor::TensorData;
use burn::tensor::backend::Backend;
use burn::{Tensor, tensor::Int};
use burn_rl::base::Action;

type ActionType = i32;

#[derive(Debug, Clone)]
pub struct GameAction {
    pub actions: Vec<ActionType>,
}

impl GameAction {
    fn new(amount: usize) -> GameAction {
        GameAction {
            actions: vec![0; amount],
        }
    }
}

impl Action for GameAction {
    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1, Int> {
        let tensor_data = TensorData::new(self.actions.clone(), [self.actions.len()]);
        Tensor::<B, 1, Int>::from_ints(tensor_data, &Default::default())
    }
}

impl From<GameAction> for Vec<ActionType> {
    fn from(action: GameAction) -> Self {
        action.actions
    }
}

impl From<Vec<ActionType>> for GameAction {
    fn from(value: Vec<ActionType>) -> Self {
        let mut game_action = GameAction::new(value.len());
        game_action.actions.copy_from_slice(&value);
        game_action
    }
}
