use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModifyRequest {
    pub token: Token,
    pub id: CrewId,
    pub loc: CrewLocation,
}
