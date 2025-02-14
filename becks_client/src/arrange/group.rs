use becks_crew::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct MatchShort {
    pub done: bool,
}

#[derive(Debug, Default)]
pub struct Group {
    pub all: Vec<Id>,
    // Matrix of [[_; len-1]; len], the subtracted 1 is self
    pub matches: Vec<Vec<MatchShort>>,
}

impl Group {
    pub fn arrange(&mut self) {
        let len = self.all.len();
        self.matches
            .resize_with(len, || vec![Default::default(); len - 1])
    }
}
