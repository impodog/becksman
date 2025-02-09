use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct UpdateRequest {
    pub token: Token,
}
