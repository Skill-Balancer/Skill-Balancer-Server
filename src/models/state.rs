use burn::tensor::Tensor;
use burn::tensor::backend::Backend;
use burn_rl::base::{ElemType, State};

const STATE_SIZE: usize = 4; // 4 should be a variable

type StateData = [ElemType; STATE_SIZE];

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    data: StateData,
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
