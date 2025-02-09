use crate::prelude::*;
use widget::{button, text, text_input};

#[derive(Default, Debug)]
pub(crate) struct LoginPanel {
    name: String,
    pass: String,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UpdateName(String),
    UpdatePass(String),
    LogIn,
    Create,
}

impl Panel for LoginPanel {
    fn update(&mut self, message: MainMessage) -> Task<MainMessage> {
        if let MainMessage::LoginMessage(message) = message {
            match message {
                LoginMessage::UpdateName(name) => {
                    self.name = name;
                    Task::none()
                }
                LoginMessage::UpdatePass(pass) => {
                    self.pass = pass;
                    Task::none()
                }
                LoginMessage::LogIn => {
                    todo!("call network interface")
                }
                LoginMessage::Create => {
                    todo!("call network interface")
                }
            }
        } else {
            Task::none()
        }
    }

    fn view(&self) -> Element<MainMessage> {
        widget::column![
            text_input(assets::TEXT.get("login_input_name"), &self.name)
                .on_input(|name| MainMessage::LoginMessage(LoginMessage::UpdateName(name))),
            text_input(assets::TEXT.get("login_input_pass"), &self.pass)
                .on_input(|pass| MainMessage::LoginMessage(LoginMessage::UpdatePass(pass))),
            button(
                text(assets::TEXT.get("login_button_login"))
                    .font(iced::Font::with_name(&config::CONFIG.assets.primary_font))
            )
            .on_press(MainMessage::LoginMessage(LoginMessage::LogIn))
            .style(button::primary),
            button(assets::TEXT.get("login_button_create"))
                .on_press(MainMessage::LoginMessage(LoginMessage::Create))
                .style(button::secondary)
        ]
        .spacing(20)
        .into()
    }
}
