use burn_rl::base::Action;

#[derive(Debug, Copy, Clone)]
pub struct GameAction {}

impl From<u32> for GameAction {
    fn from(value: u32) -> Self {
        todo!()
    }
}

impl From<GameAction> for u32 {
    fn from(action: GameAction) -> Self {
        todo!()
    }
}

impl Action for GameAction {
    fn enumerate() -> Vec<Self> {
        todo!()
    }
}
