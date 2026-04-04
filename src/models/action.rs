use rand::{thread_rng, Rng};
use burn_rl::base::Action;

#[derive(Debug, Copy, Clone)]
pub struct GameAction<const SIZE: usize> {
    actions: [usize; SIZE]
}

impl<const S: usize> Action for GameAction<S> {
    fn random(&self) -> usize {
        thread_rng().gen_range(0..self.actions.len()) as usize
    }
}
// TODO: implement the From and maybe Into functions to finally maybe make this work.