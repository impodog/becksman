use becks_crew::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct MatchShort {
    pub done: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Group {
    pub all: Vec<Id>,
}

impl Group {
    pub fn arrange(&mut self) {}
}
