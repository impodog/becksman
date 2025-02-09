use crate::prelude::*;
use crate::*;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub trait Panel: Send + Sync + std::fmt::Debug {
    fn update(&mut self, message: MainMessage) -> Task<MainMessage>;
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
    LoginMessage(login::LoginMessage),
    Login(Arc<Login>),
    Open(Arc<Mutex<Option<PanelHandle>>>),
    Rewind,
}
