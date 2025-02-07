use crate::prelude::*;
use becks_crew::*;

#[derive(Serialize, Deserialize)]
pub struct QueryByRequest {
    pub token: Token,
    pub loc: Vec<CrewLocation>,
    #[serde(default)]
    pub fuzzy: bool,
}

#[derive(Serialize, Deserialize)]
pub struct QueryByResponse {
    pub ids: Vec<Id>,
}
