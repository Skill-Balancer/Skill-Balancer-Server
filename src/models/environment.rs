use burn_rl::base::{ElemType, Snapshot, environment::Environment};

use crate::models::{action::GameAction, state::GameState};

#[derive(Debug)]
pub struct _GameValues {
    // TODO: make used at some point (it is not necessary now, but it will be)
    pub data: Vec<ElemType>,
    pub reward: ElemType,
    //why is there a reward here?
}

#[derive(Debug)]
pub struct GameEnv {
    pub state: GameState,
    pub reward: ElemType,
    pub state_size: usize,
}

impl Environment for GameEnv {
    type StateType = GameState;
    type ActionType = GameAction;
    type RewardType = ElemType;

    fn new() -> Self {
        panic!("When do we use this?") // we dont use this function anywhere it seems like we just make a new gameEnv without any function call
    }

    fn state(&self) -> Self::StateType {
        self.state.clone()
    }

    fn reset(&mut self) -> Snapshot<Self> {
        self.state = GameState::new(self.state_size);
        self.reward = 0.0;
        Snapshot::new(self.state.clone(), self.reward, false)
    }

    // All this does is send a snapshot of the current state with the reward and set the
    // DONE boolean to false
    // We are keeping action because it's a part of the function signature so it might add some errors
    // if we remove it
    fn step(&mut self, _action: Self::ActionType) -> burn_rl::base::Snapshot<Self> {
        Snapshot::new(self.state.clone(), self.reward, false)
    }
}
