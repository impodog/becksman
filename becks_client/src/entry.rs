use crate::prelude::*;
use crate::*;
use std::borrow::Cow;
use std::sync::Arc;
use widget::text;

struct Main {
    login: Option<Arc<Login>>,
    panels: Vec<PanelHandle>,
}

impl Main {
    fn init() -> Self {
        Self {
            login: None,
            panels: vec![PanelHandle::new(login::LoginPanel::default())],
        }
    }

    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::Login(login) => {
                self.login = Some(login);
                Task::none()
            }
            MainMessage::Open(panel) => {
                if let Some(panel) = panel.lock().unwrap().take() {
                    self.panels.push(panel);
                }
                Task::none()
            }
            MainMessage::Rewind => {
                self.panels.pop();
                Task::none()
            }
            _ => {
                if let Some(handle) = self.panels.last_mut() {
                    handle.update(message)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<MainMessage> {
        if let Some(handle) = self.panels.last() {
            handle.view()
        } else {
            text("Please wait...").center().size(100).into()
        }
    }
}

pub fn run_app() {
    let icon = assets::load_icon().unwrap_or_else(|err| {
        error!(
            "When loading icon from {}, {}; Using default",
            config::CONFIG.assets.icon,
            err
        );
        iced::window::icon::from_rgba(vec![], 0, 0).expect("should be able to create empty icon")
    });

    let fonts = config::CONFIG
        .assets
        .fonts
        .iter()
        .filter_map(|value| match assets::load_font_raw(value) {
            Ok(font) => Some(font),
            Err(err) => {
                error!("When reading font data from {}, {}", value, err);
                None
            }
        })
        .collect::<Vec<_>>();
    let fonts = Box::leak(Box::new(fonts));

    iced::application("Becksman", Main::update, Main::view)
        .centered()
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .theme(|_state| iced::Theme::SolarizedDark)
        .settings(iced::Settings {
            fonts: fonts
                .iter()
                .map(|font| Cow::Borrowed(font.as_slice()))
                .collect(),
            default_font: iced::Font::with_name(&config::CONFIG.assets.primary_font),
            antialiasing: true,
            ..Default::default()
        })
        .run_with(|| (Main::init(), Task::none()))
        .inspect_err(|err| {
            error!("When running main app, {}", err);
        })
        .ok();
}
