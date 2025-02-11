use crate::choices::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Id(u32);

impl Id {
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Paddle {
    pub brand: String,
    pub kind: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Rubber {
    pub brand: String,
    pub kind: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RedRubber(pub Rubber);
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlackRubber(pub Rubber);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Score(pub i32);

impl Default for Score {
    fn default() -> Self {
        Self(500)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
