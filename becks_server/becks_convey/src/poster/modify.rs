use crate::prelude::*;
use becks_poster::*;

#[derive(Serialize, Deserialize)]
pub struct ModifyRequest {
    pub token: Token,
    pub poster: Id,
    pub value: String,
}
