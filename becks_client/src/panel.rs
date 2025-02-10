use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[allow(unused_variables)]
pub trait Panel: Send + Sync + std::fmt::Debug {
    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        Task::none()
    }
    /// This method is preferred if a login is present, defaults to the prior [`Panel::update`]
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.update(message)
    }
    /// Called when rewound to the panel
    fn on_rewind_to(&mut self) -> Task<MainMessage> {
        Task::none()
    }
    /// Called on start up
    fn on_start_up(&mut self) -> Task<MainMessage> {
        Task::none()
    }
    fn view(&self) -> Element<MainMessage>;
}

#[derive(Debug)]
pub struct PanelHandle(Box<dyn Panel>);

impl PanelHandle {
    pub fn new(panel: impl Panel + 'static) -> Self {
        Self(Box::new(panel))
    }
}

impl Deref for PanelHandle {
    type Target = dyn Panel;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for PanelHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    None,
    LoginMessage(login::LoginMessage),
    LobbyMessage(lobby::LobbyMessage),
    PosterMessage(poster_panel::PosterMessage),
    Login(Arc<Login>),
    Logout,
    Open(Acquire<PanelHandle>),
    Rewind,
}
