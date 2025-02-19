use crate::prelude::*;
use crate::*;
use std::borrow::Cow;
use widget::text;

pub struct Main {
    pub login: Option<Arc<Login>>,
    panels: Vec<PanelHandle>,
}

impl Drop for Main {
    fn drop(&mut self) {
        if let Some(login) = self.login.take() {
            iced::executor::Default::new()
                .unwrap()
                .block_on(async move { login.log_out().await })
                .inspect_err(|err| error!("When logging out on program exit, {}", err))
                .ok();
        }
    }
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
                Task::done(MainMessage::Open(Acquire::new(PanelHandle::new(
                    lobby::LobbyPanel::default(),
                ))))
            }
            MainMessage::UpdateLogin => {
                if let Some(login) = self.login.clone() {
                    Task::perform(async move { login.update().await }, |_| MainMessage::None)
                } else {
                    Task::none()
                }
            }
            MainMessage::Logout => {
                if let Some(login) = self.login.take() {
                    Task::perform(async move { login.log_out().await }, |result| {
                        if let Err(err) = result {
                            error!("When logging out, {}", err);
                        }
                        MainMessage::None
                    })
                } else {
                    Task::none()
                }
            }
            MainMessage::Open(panel) => {
                if let Some(panel) = panel.try_acquire() {
                    self.panels.push(panel);
                    self.panels.last_mut().unwrap().on_start_up()
                } else {
                    Task::none()
                }
            }
            MainMessage::Rewind => {
                self.panels.pop();
                if let Some(panel) = self.panels.last_mut() {
                    panel.on_rewind_to()
                } else {
                    Task::none()
                }
            }
            MainMessage::RewindThen(panel) => {
                self.panels.pop();
                Task::done(MainMessage::Open(panel))
            }
            MainMessage::None => Task::none(),
            _ => {
                if let Some(handle) = self.panels.last_mut() {
                    if let Some(login) = self.login.as_ref() {
                        handle.update_with_login(login.clone(), message)
                    } else {
                        handle.update(message)
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<MainMessage> {
        if let Some(handle) = self.panels.last() {
            widget::column![]
                .push(handle.view())
                .push(widget::vertical_space())
                .push_maybe(if self.panels.len() <= 1 {
                    None
                } else {
                    let image = if self.panels.last().unwrap().is_done_able() {
                        widget::image("assets/done.png")
                    } else {
                        widget::image("assets/back.png")
                    };
                    Some(widget::button(image.width(20).height(20)).on_press(MainMessage::Rewind))
                })
                .into()
        } else {
            text("Please wait...").center().size(50).into()
        }
    }

    fn subscription_update_login(&self) -> iced::Subscription<MainMessage> {
        iced::time::every(config::CONFIG.request.update_relay).map(|_| MainMessage::UpdateLogin)
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
    let themes = [
        iced::Theme::TokyoNight,
        iced::Theme::Dracula,
        iced::Theme::SolarizedDark,
        iced::Theme::GruvboxDark,
        iced::Theme::Oxocarbon,
    ];
    let theme = themes
        .get(config::CONFIG.assets.theme)
        .cloned()
        .unwrap_or_default();

    iced::application(assets::TEXT.get("title"), Main::update, Main::view)
        .subscription(Main::subscription_update_login)
        .centered()
        .window(window::Settings {
            icon: Some(icon),
            ..Default::default()
        })
        .theme(move |_main| theme.clone())
        .scale_factor(|_main| 1.6)
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
