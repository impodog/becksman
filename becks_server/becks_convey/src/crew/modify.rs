use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ModifyRequest {
    pub token: Token,
    pub id: CrewId,
    pub loc: CrewLocation,
}

#[derive(Serialize, Deserialize)]
pub struct AcquireRequest {
    pub token: Token,
    pub id: CrewId,
}

#[derive(Serialize, Deserialize)]
pub struct AcquireResponse {
    pub crew: CrewData,
}
