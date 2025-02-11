use crate::prelude::*;
use becks_crew::*;

#[derive(Default, Debug, Clone)]
pub struct CrewCreatePanel {
    name: String,
    social: Option<Social>,
    error: bool,
    // Errors cause by mistakes in input
    local_error: bool,
}

#[derive(Debug, Clone)]
pub enum CrewCreateMessage {
    Start,
    Error,
    LocalError,
    UpdateName(String),
    UpdateSocial(Social),
}

impl Panel for CrewCreatePanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::CrewCreateMessage(message) => match message {
                CrewCreateMessage::Start => {
                    let this = self.clone();
                    Task::perform(
                        async move {
                            crew::CrewInfo::create(
                                login.as_ref(),
                                this.name,
                                this.social.unwrap_or_default(),
                            )
                            .await
                        },
                        |crew| match crew {
                            Ok(crew) => MainMessage::RewindThen(Acquire::new(PanelHandle::new(
                                crew_info::CrewInfoPanel::new(crew.id()),
                            ))),
                            Err(err) => {
                                warn!("When creating crew, {}", err);
                                MainMessage::CrewCreateMessage(CrewCreateMessage::Error)
                            }
                        },
                    )
                }
                CrewCreateMessage::Error => {
                    self.error = true;
                    Task::none()
                }
                CrewCreateMessage::LocalError => {
                    self.local_error = true;
                    Task::none()
                }
                CrewCreateMessage::UpdateName(name) => {
                    self.name = name;
                    Task::none()
                }
                CrewCreateMessage::UpdateSocial(social) => {
                    self.social = Some(social);
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<MainMessage> {
        widget::column![
            widget::text(assets::TEXT.get("crew_create_title")),
            widget::button(assets::TEXT.get("crew_create_create"))
                .style(widget::button::primary)
                .on_press(MainMessage::CrewCreateMessage(
                    if self.name.is_empty() || self.social.is_none() {
                        CrewCreateMessage::LocalError
                    } else {
                        CrewCreateMessage::Start
                    }
                )),
            widget::horizontal_rule(2),
            widget::text_input(assets::TEXT.get("crew_create_name_hint"), &self.name).on_input(
                |value| { MainMessage::CrewCreateMessage(CrewCreateMessage::UpdateName(value)) }
            ),
            widget::row![
                widget::text(assets::TEXT.get("crew_create_social_hint")),
                widget::pick_list(
                    Social::all_repred(),
                    self.social.as_ref().map(Social::repr),
                    |value| {
                        let social = Social::unrepr(value);
                        MainMessage::CrewCreateMessage(CrewCreateMessage::UpdateSocial(*social))
                    }
                )
            ]
        ]
        .push_maybe(if self.error {
            Some(widget::text(assets::TEXT.get("crew_create_error")).style(widget::text::danger))
        } else {
            None
        })
        .push_maybe(if self.local_error {
            Some(
                widget::text(assets::TEXT.get("crew_create_localerror"))
                    .style(widget::text::danger),
            )
        } else {
            None
        })
        .spacing(10)
        .padding(20)
        .into()
    }
}
