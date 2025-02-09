use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Poster {
    pub value: String,
    pub compiled: String,
    pub timestamp: u64,
}
