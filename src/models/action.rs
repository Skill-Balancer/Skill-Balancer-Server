use burn::tensor::TensorData;
use burn::tensor::backend::Backend;
use burn::{Tensor, tensor::Int};
use burn_rl::base::Action;

type ActionType = i32;

#[derive(Debug, Clone)]
pub struct GameAction {
    pub actions: Vec<ActionType>,
    index: ActionType,
}

impl GameAction {
    fn new(amount: usize) -> GameAction {
        GameAction {
            actions: vec![0; amount],
            index: 0,
        }
    }

    fn categorical_index(actions: &[ActionType]) -> ActionType {
        for (position, value) in actions.iter().enumerate() {
            match *value {
                1 => return position as ActionType * 2 + 1,
                -1 => return position as ActionType * 2 + 2,
                _ => {}
            }
        }
        0
    }
}

impl Action for GameAction {
    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1, Int> {
        let tensor_data = TensorData::new(vec![self.index], [1]);
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
        game_action.index = GameAction::categorical_index(&value);
        game_action.actions.copy_from_slice(&value);
        game_action
    }
}
