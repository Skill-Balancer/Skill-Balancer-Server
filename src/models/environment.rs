use burn_rl::base::{ElemType, environment::Environment};

use crate::models::{action::GameAction, state::GameState};

#[derive(Debug)]
pub struct GameValues {}


#[derive(Debug)]
pub struct GameEnv {
    state: GameState,
    reward: f32
}



impl GameEnv {
    // The intention of this function is to set this data to the data that comes from the HTTP request
    // from the client. I.E we need to use this function in main.
    pub fn setDataFromClient(&mut self, result: GameValues)
        self.state = GameState::from(result);
        self.state = GameState::from(reward);
}


impl Environment for GameEnv {
    type StateType = GameState;
    type ActionType = GameAction;
    type RewardType = ElemType;

    fn new() -> Self {
        self.state = GameEnv::new();
    }

    fn state(&self) -> Self::StateType {
        self.state
    }

    fn reset(&mut self) -> burn_rl::base::Snapshot<Self> {
        self.state = GameState::new(); 
        Snapshot::new(self.state, 0, false);
    }


    // All this does is send a snapshot of the current state with the reward and set the
    // DONE boolean to false
    // We are keeping action because it's a part of the function signature so it might add some errors
    // if we remove it
    fn step(&mut self, action: Self::ActionType) -> burn_rl::base::Snapshot<Self> {
        Snapshot::new(self.state, self.reward, false);
    }
}
