use crate::choices::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct CrewId(u32);

impl CrewId {
    /// Generates a random but not (very) unique user id
    pub fn rand() -> Self {
        Self(rand::random())
    }

    pub fn to_prim(&self) -> u32 {
        self.0
    }

    pub fn from_prim(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Paddle {
    pub brand: String,
    pub kind: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Rubber {
    pub brand: String,
    pub kind: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RedRubber(pub Rubber);
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct BlackRubber(pub Rubber);

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Score(pub i32);

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CrewData {
    pub name: String,
    pub social: Social,
    pub score: Score,
    pub gender: Option<Gender>,
    pub clothes: Option<Clothes>,
    pub hand: Option<Hand>,
    pub hold: Option<Hold>,
    pub paddle: Option<Paddle>,
    pub red: Option<RedRubber>,
    pub black: Option<BlackRubber>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CrewLocation {
    Name(String),
    Social(Social),
    Score(Score),
    Gender(Gender),
    Clothes(Clothes),
    Hand(Hand),
    Hold(Hold),
    Paddle(Paddle),
    Red(RedRubber),
    Black(BlackRubber),
}
