use crate::prelude::*;
use widget::{button, text};

#[derive(Default, Debug)]
pub(crate) struct LobbyPanel {
    pub poster: poster::PosterList,
}

#[derive(Debug, Clone)]
pub enum LobbyMessage {
    LoadRecentPoster,
    LoededRecentPoster(Acquire<poster::PosterList>),
}
