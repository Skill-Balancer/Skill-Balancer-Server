use burn_rl::base::{ElemType, Snapshot, environment::Environment};

use crate::models::{action::GameAction, state::GameState};

#[derive(Debug)]
pub struct GameValues {
    pub data: [ElemType; 4],
    pub reward: ElemType,
}

#[derive(Debug)]
pub struct GameEnv {
    pub state: GameState,
    pub reward: ElemType,
}

impl GameEnv {
    // Set the data from HTTP from the client.
    // This isn't used yet in main
    pub fn set_data_from_client(&mut self, input: GameValues) {
        self.state = GameState::from(input.data);
        self.reward = input.reward;
    }
}

impl Environment for GameEnv {
    type StateType = GameState;
    type ActionType = GameAction;
    type RewardType = ElemType;

    fn new() -> Self {
        Self {
            state: GameState::new(),
            reward: 0.0,
        }
    }

    fn state(&self) -> Self::StateType {
        self.state
    }

    fn reset(&mut self) -> Snapshot<Self> {
        self.state = GameState::new();
        self.reward = 0.0;
        Snapshot::new(self.state.clone(), self.reward, false)
    }

    // All this does is send a snapshot of the current state with the reward and set the
    // DONE boolean to false
    // We are keeping action because it's a part of the function signature so it might add some errors
    // if we remove it
    fn step(&mut self, _action: Self::ActionType) -> burn_rl::base::Snapshot<Self> {
        Snapshot::new(self.state, self.reward, false)
    }
}
