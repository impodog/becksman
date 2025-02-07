use crate::prelude::*;
use becks_poster::*;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub token: Token,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    pub poster: Id,
}
