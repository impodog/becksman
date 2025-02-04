use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModifyRequest {
    pub token: Token,
    pub id: CrewId,
    pub loc: ModifyLocation,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ModifyLocation {
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
