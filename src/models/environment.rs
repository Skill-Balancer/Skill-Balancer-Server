use burn_rl::base::{ElemType, environment::Environment};

use crate::models::{action::GameAction, state::GameState};

#[derive(Debug)]
pub struct GameEnv {
    state: GameState,
}

impl Environment for GameEnv {
    type StateType = GameState;
    type ActionType = GameAction;
    type RewardType = ElemType;

    fn new() -> Self {
        todo!()
    }

    fn state(&self) -> Self::StateType {
        self.state
    }

    fn reset(&mut self) -> burn_rl::base::Snapshot<Self> {
        todo!()
    }

    fn step(&mut self, action: Self::ActionType) -> burn_rl::base::Snapshot<Self> {
        todo!()
    }
}
