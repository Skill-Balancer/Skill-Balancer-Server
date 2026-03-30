use burn_rl::base::Action;

#[derive(Debug, Copy, Clone)]
pub enum GameAction {
    NoChange,
    PlayerOneAttInc,
    PlayerOneAttDec,
    PlayerTwoAttInc,
    PlayerTwoAttDec,
}

impl From<u32> for GameAction {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::NoChange,
            1 => Self::PlayerOneAttInc,
            2 => Self::PlayerOneAttDec,
            3 => Self::PlayerTwoAttInc,
            4 => Self::PlayerTwoAttDec,
            _ => panic!("This action does not exist!"),
        }
    }
}

impl Into<isize> for GameAction {
    fn into(self) -> isize {
        match self {
            GameAction::NoChange => 0,
            GameAction::PlayerOneAttInc => 1,
            GameAction::PlayerOneAttDec => -1,
            GameAction::PlayerTwoAttInc => 2,
            GameAction::PlayerTwoAttDec => -2,
        }
    }
}

impl From<GameAction> for u32 {
    fn from(action: GameAction) -> Self {
        match action {
            GameAction::NoChange => 0,
            GameAction::PlayerOneAttInc => 1,
            GameAction::PlayerOneAttDec => 2,
            GameAction::PlayerTwoAttInc => 3,
            GameAction::PlayerTwoAttDec => 4,
        }
    }
}

impl Action for GameAction {
    fn enumerate() -> Vec<Self> {
        vec![
            Self::NoChange,
            Self::PlayerOneAttInc,
            Self::PlayerOneAttDec,
            Self::PlayerTwoAttDec,
            Self::PlayerTwoAttInc,
        ]
    }
}
