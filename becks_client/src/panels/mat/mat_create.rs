use crate::prelude::*;
use becks_match::*;

#[derive(Debug)]
pub struct MatCreatePanel {
    mat: Match,
}

#[derive(Debug, Clone)]
pub enum MatCreateMessage {
    StartCreate,
}
