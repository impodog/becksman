use crate::prelude::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct Round {
    pub left_win: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Quit {
    #[default]
    Normal,
    LeftQuit,
    RightQuit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    pub total_rounds: usize,
    pub left: Id,
    pub right: Id,
    pub round_worth: u32,
    pub rounds: Vec<Round>,
    pub timestamp: u64,
    #[serde(default)]
    pub quit: Quit,
    #[serde(default)]
    pub notes: String,
}

impl Match {
    pub fn new(total_rounds: usize, left: Id, right: Id, timestamp: u64) -> Self {
        Self {
            total_rounds,
            left,
            right,
            round_worth: (total_rounds as u32 * 10).div_ceil(3),
            rounds: Default::default(),
            timestamp,
            quit: Default::default(),
            notes: Default::default(),
        }
    }
}
