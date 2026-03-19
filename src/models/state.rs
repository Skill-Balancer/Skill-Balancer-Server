use burn::tensor::Tensor;
use burn::tensor::backend::Backend;
use burn_rl::base::{ElemType, State};

const STATE_SIZE: usize = 4; // 4 should be a variable

type StateData = [ElemType; STATE_SIZE];

// data has to be public so environment can access it
#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub(crate) data: StateData,
}

impl From<StateData> for GameState {
    fn from(data: StateData) -> Self {
        Self { data }
    }
}

// Just used for implementing new GameStates. At a later point we should
// Figure out what initial values are good for implementing the gamestate with.
impl GameState {
    pub fn new() -> Self {
        Self { data: [0.5; STATE_SIZE] }
    }
}

impl State for GameState {
    type Data = StateData;

    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1> {
        Tensor::<B, 1>::from_floats(self.data, &Default::default())
    }

    fn size() -> usize {
        STATE_SIZE
    }
}
