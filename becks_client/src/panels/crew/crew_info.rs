use crate::{entry::Main, prelude::*};
use becks_crew::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct CrewInfoPanel {
    crew: Arc<Mutex<crew::CrewInfo>>,
    crew_data: Option<CrewData>,
    error: bool,
}

#[derive(Debug, Clone)]
pub enum CrewInfoMessage {
    Load,
    Loaded(Acquire<CrewData>),
    LoadError,
    Update(CrewLocation),
}

impl CrewInfoPanel {
    pub fn new(id: Id) -> Self {
        Self {
            crew: Arc::new(Mutex::new(crew::CrewInfo::new(id))),
            crew_data: None,
            error: false,
        }
    }
}

impl Panel for CrewInfoPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::CrewInfoMessage(message) => match message {
                CrewInfoMessage::Load => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move { crew.lock().await.load(login.as_ref()).await.cloned() },
                        |crew| match crew {
                            Ok(crew_data) => MainMessage::CrewInfoMessage(CrewInfoMessage::Loaded(
                                Acquire::new(crew_data),
                            )),
                            Err(err) => {
                                warn!("When loading crew message, {}", err);
                                MainMessage::CrewInfoMessage(CrewInfoMessage::LoadError)
                            }
                        },
                    )
                }
                CrewInfoMessage::Loaded(crew_data) => {
                    if let Some(crew_data) = crew_data.try_acquire() {
                        self.crew_data = Some(crew_data);
                        self.error = false;
                    }
                    Task::none()
                }
                CrewInfoMessage::Update(loc) => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move { crew.lock().await.modify(login.as_ref(), loc).await },
                        |result| {
                            if let Err(err) = result {
                                error!("When modifying crew {}", err);
                            }
                            MainMessage::CrewInfoMessage(CrewInfoMessage::Load)
                        },
                    )
                }
                CrewInfoMessage::LoadError => {
                    self.error = true;
                    Task::none()
                }
            },
            _ => {
                todo!()
            }
        }
    }

    fn view(&self) -> Element<MainMessage> {
        let mut column: Vec<Element<MainMessage>> = Vec::new();
        column.push(widget::text(assets::TEXT.get("crew_info_title")).into());
        if let Some(data) = self.crew_data.as_ref() {
            column.push(view_data(
                "crew_info_name",
                widget::text_input(assets::TEXT.get("crew_info_name"), &data.name).on_input(
                    |name| {
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Name(
                            name,
                        )))
                    },
                ),
            ));
            column.push(view_data(
                "crew_info_social",
                widget::pick_list(Social::all_repred(), Some(data.social.repr()), |social| {
                    let social = Social::unrepr(social);
                    MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Social(
                        *social,
                    )))
                }),
            ));
        } else {
            column.push(widget::text(assets::TEXT.get("crew_loading")).into());
        }
        if self.error {
            column.push(widget::horizontal_rule(2).into());
            column.push(
                widget::text("crew_info_error")
                    .style(widget::text::danger)
                    .into(),
            );
        }
        widget::Column::from_iter(column).spacing(10).into()
    }

    fn on_start_up(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::CrewInfoMessage(CrewInfoMessage::Load))
    }

    fn is_done_able(&self) -> bool {
        true
    }
}

fn view_data_key(key: &str) -> Element<'static, MainMessage> {
    widget::text(assets::TEXT.get(key))
        .style(widget::text::primary)
        .into()
}

fn view_data<'a>(key: &str, data: impl Into<Element<'a, MainMessage>>) -> Element<'a, MainMessage> {
    widget::row![view_data_key(key), widget::horizontal_space(), data.into()].into()
}
