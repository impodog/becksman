use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize)]
pub struct QueryByRequest {
    pub token: Token,
    pub loc: CrewLocation,
}

#[derive(Serialize, Deserialize)]
pub struct QueryByResponse {
    pub ids: Vec<CrewId>,
}
