use crate::prelude::*;
use becks_poster::*;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize)]
pub enum QueryPosterBy {
    Content(String),
    Time { mid: u64, error: u64 },
}

#[derive(Serialize, Deserialize)]
pub struct QueryRequest {
    pub token: Token,
    pub by: Vec<QueryPosterBy>,
}
impl Deref for QueryRequest {
    type Target = Vec<QueryPosterBy>;
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
