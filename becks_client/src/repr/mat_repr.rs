use crate::prelude::*;
use becks_match::*;

impl Repr for Quit {
    fn repr(&self) -> &'static str {
        match self {
            Self::Normal => assets::TEXT.get("quit_normal"),
            Self::LeftQuit => assets::TEXT.get("quit_left"),
            Self::RightQuit => assets::TEXT.get("quit_right"),
        }
    }
    fn all() -> &'static [Self] {
        &[Self::Normal, Self::LeftQuit, Self::RightQuit]
    }
}
