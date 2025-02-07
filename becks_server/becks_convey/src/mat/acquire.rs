use crate::prelude::*;
use becks_match::*;

#[derive(Serialize, Deserialize)]
pub struct AcquireRequest {
    pub token: Token,
    pub mat: Id,
}

#[derive(Serialize, Deserialize)]
pub struct AcquireResponse {
    pub mat: Match,
}
