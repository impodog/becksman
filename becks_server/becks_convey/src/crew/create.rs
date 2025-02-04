use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub token: Token,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    pub id: CrewId,
}
