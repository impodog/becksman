use crate::prelude::*;
use becks_match::*;

#[derive(Serialize, Deserialize)]
pub struct ModifyRequest {
    pub token: Token,
    pub mat: Id,
    pub notes: String,
}
