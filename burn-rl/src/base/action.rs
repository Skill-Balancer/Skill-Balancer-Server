use std::fmt::Debug;

pub trait Action: Debug + Copy + Clone + From<u32> + Into<u32> {
    fn random(&self) -> usize;
}
