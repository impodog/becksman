use crate::choices::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct CrewId(u64);

impl CrewId {
    /// Generates a random but not (very) unique user id
    pub fn rand() -> Self {
        Self(rand::random())
    }

    pub fn to_prim(&self) -> u64 {
        self.0
    }

    pub fn from_prim(value: u64) -> Self {
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

#[derive(Default)]
pub struct CrewData {
    pub name: String,
    pub social: Social,
    pub gender: Gender,
    pub clothes: Clothes,
    pub hand: Hand,
    pub hold: Hold,
    pub paddle: Paddle,
    pub red: RedRubber,
    pub black: BlackRubber,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CrewLocation {
    Name(String),
    Social(Social),
    Gender(Gender),
    Clothes(Clothes),
    Hand(Hand),
    Hold(Hold),
    Paddle(Paddle),
    Red(RedRubber),
    Black(BlackRubber),
}
