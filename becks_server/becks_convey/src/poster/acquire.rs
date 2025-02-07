use crate::prelude::*;
use becks_poster::*;

#[derive(Serialize, Deserialize)]
pub struct AcquireRequest {
    pub token: Token,
    pub poster: Id,
}

#[derive(Serialize, Deserialize)]
pub struct AcquireResponse {
    pub poster: Poster,
}
