use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize)]
pub struct DeleteRequest {
    pub token: Token,
    pub id: CrewId,
}
