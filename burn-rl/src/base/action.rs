use std::fmt::Debug;

use burn::tensor::backend::Backend;
use burn::tensor::Int;
use burn::Tensor;

pub trait Action: Debug + Clone + From<Vec<i32>> + Into<Vec<i32>> {
    fn to_tensor<B: Backend>(&self) -> Tensor<B, 1, Int>;
}
