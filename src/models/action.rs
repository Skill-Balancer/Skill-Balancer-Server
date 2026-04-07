use burn::{Tensor, tensor::Int};
use rand::{thread_rng, Rng};
use burn_rl::base::Action;
use burn::tensor::backend::Backend;

type ActionType = usize;

#[derive(Debug, Clone)]
pub struct GameAction {
    pub actions: Vec<ActionType>,
    pub size: usize,
}

impl GameAction {
    fn new(amount : usize) -> GameAction {
        GameAction { actions: vec![0; amount], size: amount}
    }
}

impl Action for GameAction {    
    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1, Int> {
        Tensor::<B, 1, Int>::from_ints(self.actions, &Default::default())
    }
}

impl From<GameAction> for u32 {
    fn from(action: GameAction) -> Self {
        5
    }
}

impl From<u32> for GameAction {
    fn from(mut value: u32) -> Self {
        if value == 0 {
            return GameAction::new(5);
        }

        let increase = if value % 2 == 0 {
            true
        } else {
            false
        };

        let index = (value as f32 / 2.0).ceil(); // unfinished

        let game_actions = GameAction::new();

        game_actions
    }
}