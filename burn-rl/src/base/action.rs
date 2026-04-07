use std::fmt::Debug;

use burn::Tensor;
use burn::tensor::Int;
use burn::tensor::backend::Backend;

pub trait Action: Debug + Clone + From<u32> + Into<u32> {
    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1, Int>;
}
