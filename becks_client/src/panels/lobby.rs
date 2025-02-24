use crate::prelude::*;
use widget::{button, text};

#[derive(Default, Debug)]
pub(crate) struct LobbyPanel {
    pub poster: poster_panel::PosterPanel,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LobbyMessage {
    LoadRecentPoster,
    LoadedRecentPoster(Acquire<poster::PosterList>),
    LoadRecentPosterError(String),
}

impl Panel for LobbyPanel {
    fn update(&mut self, _message: MainMessage) -> Task<MainMessage> {
        Task::done(MainMessage::Rewind)
    }
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = None;
        match message {
            MainMessage::LobbyMessage(message) => match message {
                LobbyMessage::LoadRecentPoster => Task::perform(
                    async move {
                        poster::PosterList::query(
                            login.as_ref(),
                            vec![poster::methods::query::QueryPosterBy::Time {
                                mid: current_timestamp(),
                                error: config::CONFIG.interact.recent.as_secs(),
                            }],
                        )
                        .await
                    },
                    |result| match result {
                        Ok(poster) => MainMessage::LobbyMessage(LobbyMessage::LoadedRecentPoster(
                            Acquire::new(poster),
                        )),
                        Err(err) => {
                            error!("When loading recent poster, {}", err);
                            MainMessage::LobbyMessage(LobbyMessage::LoadRecentPosterError(
                                err.to_string(),
                            ))
                        }
                    },
                ),
                LobbyMessage::LoadedRecentPoster(poster) => {
                    if let Some(poster) = poster.try_acquire() {
                        self.poster = poster_panel::PosterPanel::new(poster, false);
                        Task::done(MainMessage::PosterMessage(
                            poster_panel::PosterMessage::Reload,
                        ))
                    } else {
                        Task::none()
                    }
                }
                LobbyMessage::LoadRecentPosterError(error) => {
                    self.error = Some(error);
                    Task::none()
                }
            },
            _ => self.poster.update_with_login(login, message),
        }
    }
    fn view(&self) -> Element<MainMessage> {
        // TODO: Lobby Elements
        widget::column![
            widget::row![
                widget::button(assets::TEXT.get("lobby_crew"))
                    .style(widget::button::text)
                    .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                        crew_query::CrewQueryPanel::default()
                    )))),
                widget::button(assets::TEXT.get("lobby_poster"))
                    .style(widget::button::text)
                    .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                        poster_query::PosterQueryPanel::default()
                    )))),
                widget::button(assets::TEXT.get("lobby_mat"))
                    .style(widget::button::text)
                    .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                        mat_create::MatCreatePanel::default()
                    )))),
                widget::button(assets::TEXT.get("lobby_arrange"))
                    .style(widget::button::text)
                    .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                        mat_arrange::MatArrangePanel::default()
                    ))))
            ]
            .spacing(30),
            widget::container(self.poster.view()).style(widget::container::rounded_box),
        ]
        .into()
    }
    fn on_start_up(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::LobbyMessage(LobbyMessage::LoadRecentPoster))
    }
}
