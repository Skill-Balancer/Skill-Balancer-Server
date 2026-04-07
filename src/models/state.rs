use burn::tensor::Tensor;
use burn::tensor::backend::Backend;
use burn_rl::base::{ElemType, State};

// data has to be public so environment can access it
#[derive(Debug, Clone)]
pub struct GameState {
    pub data: Vec<ElemType>,
}

impl From<Vec<ElemType>> for GameState {
    fn from(data: Vec<ElemType>) -> Self {
        Self { data }
    }
}

// Just used for implementing new GameStates. At a later point we should
// Figure out what initial values are good for implementing the gamestate with.
impl GameState {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size]
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl State for GameState {
    type Data = Vec<ElemType>;

    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1> {
        Tensor::<B, 1>::from_floats(self.data.as_slice(), &Default::default())
    }

    fn size() -> usize {
        panic!("dont use this");
        // it expected the pre defined size but we dont have that anymore
    }

}
