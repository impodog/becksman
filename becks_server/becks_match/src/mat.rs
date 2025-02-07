use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Round {
    pub left_win: bool,
}

#[derive(Serialize, Deserialize, Default, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Quit {
    #[default]
    Normal,
    LeftQuit,
    RightQuit,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Match {
    pub total_rounds: u16,
    pub left: Id,
    pub right: Id,
    pub round_worth: u32,
    pub rounds: Vec<Round>,
    #[serde(default)]
    pub quit: Quit,
    #[serde(default)]
    pub notes: String,
}

impl Match {
    pub fn new(total_rounds: u16, left: Id, right: Id) -> Self {
        Self {
            total_rounds,
            left,
            right,
            round_worth: (total_rounds as u32 * 5).div_ceil(3),
            rounds: Default::default(),
            quit: Default::default(),
            notes: Default::default(),
        }
    }
}
