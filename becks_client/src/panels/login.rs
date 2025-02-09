use crate::prelude::*;
use std::sync::{Arc, Mutex};
use widget::{button, text, text_input};

#[derive(Default, Debug)]
pub(crate) struct LoginPanel {
    name: String,
    pass: String,
    error: Arc<Mutex<ErrorStatus>>,
}

#[derive(Default, Debug)]
enum ErrorStatus {
    #[default]
    None,
    Login,
    Create,
}

#[derive(Debug, Clone)]
pub enum LoginMessage {
    UpdateName(String),
    UpdatePass(String),
    LogIn,
    Create,
}

async fn login_in_by(name: String, pass: String) -> Result<login::Login> {
    login::Login::log_in(name, pass).await
}

async fn create_by(name: String, pass: String) -> Result<Result<login::Login>> {
    login::Login::create(name, pass).await
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
                    let error = self.error.clone();
                    Task::perform(
                        login_in_by(self.name.clone(), self.pass.clone()),
                        move |login| match login {
                            Ok(login) => {
                                *error.lock().unwrap() = ErrorStatus::None;
                                MainMessage::Login(Arc::new(login))
                            }
                            Err(err) => {
                                warn!("When logging in, {}", err);
                                *error.lock().unwrap() = ErrorStatus::Login;
                                MainMessage::None
                            }
                        },
                    )
                }
                LoginMessage::Create => {
                    let error = self.error.clone();
                    Task::perform(
                        create_by(self.name.clone(), self.pass.clone()),
                        move |login| match login {
                            Ok(login) => match login {
                                Ok(login) => {
                                    *error.lock().unwrap() = ErrorStatus::None;
                                    MainMessage::Login(Arc::new(login))
                                }
                                Err(err) => {
                                    warn!("When logging in after creation, {}", err);
                                    *error.lock().unwrap() = ErrorStatus::Login;
                                    MainMessage::None
                                }
                            },
                            Err(err) => {
                                warn!("When creating user, {}", err);
                                *error.lock().unwrap() = ErrorStatus::Create;
                                MainMessage::None
                            }
                        },
                    )
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
                .style(button::secondary),
        ]
        .push_maybe(match *self.error.lock().unwrap() {
            ErrorStatus::None => None,
            ErrorStatus::Login => Some(
                text(assets::TEXT.get("login_error_login"))
                    .color(iced::Color::from_rgb8(255, 50, 50)),
            ),
            ErrorStatus::Create => Some(
                text(assets::TEXT.get("login_error_create"))
                    .color(iced::Color::from_rgb8(255, 50, 50)),
            ),
        })
        .spacing(20)
        .into()
    }

    fn on_rewind_to(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::Logout)
    }
}
