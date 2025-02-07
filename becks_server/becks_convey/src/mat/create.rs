use crate::prelude::*;
use becks_match::*;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub token: Token,
    pub mat: Match,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    pub mat: Id,
}
