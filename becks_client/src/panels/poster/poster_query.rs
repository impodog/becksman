use crate::prelude::*;
use becks_poster::*;
use poster::methods::query::QueryPosterBy;

#[derive(Debug, Clone, Copy, Default)]
pub struct TimeInterval {
    mid: Option<u64>,
    error: Option<u64>,
}
impl From<TimeInterval> for QueryPosterBy {
    fn from(value: TimeInterval) -> Self {
        let mid = value.mid.unwrap_or_else(current_timestamp);
        let error = value
            .error
            .unwrap_or_else(|| config::CONFIG.interact.recent.as_secs());
        QueryPosterBy::Time { mid, error }
    }
}

impl TimeInterval {
    fn value_to_string(value: Option<&u64>) -> String {
        value.map(ToString::to_string).unwrap_or_default()
    }

    fn view(&self) -> Element<MainMessage> {
        widget::row![
            widget::text_input(
                assets::TEXT.get("poster_query_time_mid"),
                &Self::value_to_string(self.mid.as_ref())
            )
            .on_input(|value| {
                if value.is_empty() {
                    MainMessage::PosterQueryMessage(PosterQueryMessage::ModifyTime(TimeInterval {
                        mid: None,
                        error: self.error,
                    }))
                } else if let Ok(value) = value.parse() {
                    MainMessage::PosterQueryMessage(PosterQueryMessage::ModifyTime(TimeInterval {
                        mid: Some(value),
                        error: self.error,
                    }))
                } else {
                    MainMessage::None
                }
            }),
            widget::text_input(
                assets::TEXT.get("poster_query_time_error"),
                &Self::value_to_string(self.error.as_ref())
            )
            .on_input(|value| {
                if value.is_empty() {
                    MainMessage::PosterQueryMessage(PosterQueryMessage::ModifyTime(TimeInterval {
                        mid: self.mid,
                        error: None,
                    }))
                } else if let Ok(value) = value.parse() {
                    MainMessage::PosterQueryMessage(PosterQueryMessage::ModifyTime(TimeInterval {
                        mid: self.mid,
                        error: Some(value),
                    }))
                } else {
                    MainMessage::None
                }
            }),
        ]
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum PosterQueryMessage {
    AddKeyword,
    ModifyKeyword(usize, String),
    ModifyTime(TimeInterval),
    StartQuery,
    QueryDone(Acquire<poster_panel::PosterPanel>),
    QueryError,
}

#[derive(Debug, Default)]
pub struct PosterQueryPanel {
    keywords: Vec<String>,
    time: TimeInterval,
    list: Option<poster_panel::PosterPanel>,
    error: bool,
}

impl Panel for PosterQueryPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        match message {
            MainMessage::PosterQueryMessage(message) => match message {
                PosterQueryMessage::AddKeyword => {
                    self.keywords.push(String::default());
                    Task::none()
                }
                PosterQueryMessage::ModifyTime(time) => {
                    self.time = time;
                    Task::none()
                }
                PosterQueryMessage::ModifyKeyword(pos, value) => {
                    if let Some(keyword) = self.keywords.get_mut(pos) {
                        *keyword = value;
                    }
                    Task::none()
                }
                PosterQueryMessage::StartQuery => {
                    let mut by = Vec::new();
                    let timeless = !self.keywords.is_empty();
                    by.extend(
                        self.keywords
                            .iter()
                            .filter_map(|keyword| {
                                let keyword = keyword.trim();
                                if keyword.is_empty() {
                                    None
                                } else {
                                    Some(keyword.to_owned())
                                }
                            })
                            .map(QueryPosterBy::Content),
                    );
                    if !timeless {
                        by.push(self.time.into());
                    }
                    Task::perform(
                        async move { poster::PosterList::query(login.as_ref(), by).await },
                        move |result| match result {
                            Ok(list) => {
                                let list = poster_panel::PosterPanel::new(list, timeless);
                                MainMessage::PosterQueryMessage(PosterQueryMessage::QueryDone(
                                    Acquire::new(list),
                                ))
                            }
                            Err(err) => {
                                warn!("When querying for posters, {}", err);
                                MainMessage::PosterQueryMessage(PosterQueryMessage::QueryError)
                            }
                        },
                    )
                }
                PosterQueryMessage::QueryDone(list) => {
                    if let Some(list) = list.try_acquire() {
                        self.list = Some(list);
                        Task::done(MainMessage::PosterMessage(
                            poster_panel::PosterMessage::Load,
                        ))
                    } else {
                        Task::none()
                    }
                }
                PosterQueryMessage::QueryError => {
                    self.error = true;
                    Task::none()
                }
            },
            _ => {
                if let Some(list) = self.list.as_mut() {
                    list.update_with_login(login, message)
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> Element<MainMessage> {
        widget::column![
            widget::text(assets::TEXT.get("poster_query_title")),
            widget::button(if self.keywords.is_empty() {
                assets::TEXT.get("poster_query_acquire")
            } else {
                assets::TEXT.get("poster_query_start")
            })
            .on_press(MainMessage::PosterQueryMessage(
                PosterQueryMessage::StartQuery
            )),
            view_keywords(self.keywords.as_slice()),
            // TODO: Allow time viewing later
            // self.time.view(),
        ]
        .push_maybe(self.list.as_ref().map(|list| list.view()))
        .push_maybe(if self.error {
            Some(widget::text(assets::TEXT.get("poster_query_error")).style(widget::text::danger))
        } else {
            None
        })
        .push(widget::horizontal_rule(2))
        .push(
            widget::button(assets::TEXT.get("poster_query_create")).on_press(MainMessage::Open(
                Acquire::new(PanelHandle::new(poster_create::PosterCreatePanel::default())),
            )),
        )
        .spacing(10)
        .into()
    }
}

fn view_keywords(keywords: &[String]) -> Element<MainMessage> {
    let mut row: Vec<Element<MainMessage>> = Vec::new();
    row.push(
        widget::text(assets::TEXT.get("poster_query_keyword_hint"))
            .style(widget::text::primary)
            .into(),
    );
    for (index, keyword) in keywords.iter().enumerate() {
        row.push(
            widget::text_input(assets::TEXT.get("poster_query_keyword_hint"), keyword)
                .on_input(move |value| {
                    MainMessage::PosterQueryMessage(PosterQueryMessage::ModifyKeyword(index, value))
                })
                .into(),
        )
    }
    row.push(
        widget::button(widget::image("assets/add.png").height(20))
            .on_press(MainMessage::PosterQueryMessage(
                PosterQueryMessage::AddKeyword,
            ))
            .into(),
    );
    widget::Row::from_iter(row).into()
}
