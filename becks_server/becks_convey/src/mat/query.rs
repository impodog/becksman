use crate::prelude::*;
use becks_match::*;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize)]
pub enum QueryMatchBy {
    Player(Id),
    Note(String),
    Time { mid: u64, error: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct QueryRequest {
    pub token: Token,
    pub by: Vec<QueryMatchBy>,
}
impl Deref for QueryRequest {
    type Target = Vec<QueryMatchBy>;
    fn deref(&self) -> &Self::Target {
        &self.by
    }
}
impl DerefMut for QueryRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.by
    }
}

#[derive(Serialize, Deserialize)]
pub struct QueryResponse {
    pub ids: Vec<Id>,
}
