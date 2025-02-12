use crate::prelude::*;

#[derive(Debug, Default)]
pub struct PosterPanel {
    poster: Arc<poster::PosterList>,
    loaded: Vec<becks_poster::Poster>,
    is_loaded: bool,
    timeless: bool,
}

impl PosterPanel {
    pub fn new(poster: poster::PosterList, timeless: bool) -> Self {
        Self {
            poster: Arc::new(poster),
            loaded: Default::default(),
            is_loaded: false,
            timeless,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PosterMessage {
    Reload,
    Load,
    Loaded(Acquire<Vec<becks_poster::Poster>>),
    View(usize),
}

impl Panel for PosterPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::PosterMessage(message) => match message {
                PosterMessage::Load => {
                    let poster = self.poster.clone();
                    Task::perform(
                        async move {
                            let mut posters = Vec::new();
                            for poster in poster.iter() {
                                let poster =
                                    poster.write().await.load(login.as_ref()).await.cloned();
                                posters.push(poster)
                            }
                            posters
                        },
                        |posters| {
                            let posters = posters.into_iter().collect::<Result<Vec<_>>>();
                            match posters {
                                Ok(mut posters) => {
                                    posters.sort_unstable_by(|lhs, rhs| {
                                        lhs.timestamp.cmp(&rhs.timestamp)
                                    });
                                    MainMessage::PosterMessage(PosterMessage::Loaded(Acquire::new(
                                        posters,
                                    )))
                                }
                                Err(err) => {
                                    error!("When loading posters, {}", err);
                                    MainMessage::None
                                }
                            }
                        },
                    )
                }
                PosterMessage::Reload => {
                    self.loaded.clear();
                    self.is_loaded = false;
                    let poster = self.poster.clone();
                    Task::perform(
                        async move { poster.reload(login.as_ref()).await },
                        |result| {
                            if let Err(err) = result {
                                error!("When reloading posters, {}", err);
                            }
                            MainMessage::PosterMessage(PosterMessage::Load)
                        },
                    )
                }
                PosterMessage::Loaded(loaded) => {
                    if let Some(loaded) = loaded.try_acquire() {
                        self.loaded = loaded;
                        self.is_loaded = true;
                    }
                    Task::none()
                }
                PosterMessage::View(index) => {
                    let poster = self.poster.clone();
                    Task::perform(
                        async move {
                            if let Some(poster) = poster.get(index) {
                                Some(poster.read().await.clone())
                            } else {
                                None
                            }
                        },
                        |poster| {
                            if let Some(poster) = poster {
                                MainMessage::Open(Acquire::new(PanelHandle::new(
                                    poster_view::PosterViewPanel::new(poster),
                                )))
                            } else {
                                MainMessage::None
                            }
                        },
                    )
                }
            },
            _ => Task::none(),
        }
    }
    fn view(&self) -> Element<MainMessage> {
        let poster_view: Element<MainMessage> = if self.is_loaded {
            if self.poster.is_empty() {
                widget::text(if self.timeless {
                    assets::TEXT.get("poster_empty_timeless")
                } else {
                    assets::TEXT.get("poster_empty")
                })
                .into()
            } else {
                let mut column: Vec<Element<MainMessage>> = Vec::new();
                for (index, poster) in self.loaded.iter().enumerate() {
                    column.push(view_poster(poster, true));
                    column.push(
                        widget::button(widget::image("assets/jump.png"))
                            .height(25)
                            .on_press(MainMessage::PosterMessage(PosterMessage::View(index)))
                            .into(),
                    );
                    column.push(widget::Rule::horizontal(2).into());
                }
                widget::scrollable(widget::Column::from_iter(column))
                    .height(300)
                    .into()
            }
        } else {
            widget::text(assets::TEXT.get("poster_loading"))
                .style(widget::text::secondary)
                .into()
        };
        widget::column![
            widget::text(assets::TEXT.get("poster_title")).style(widget::text::primary),
            poster_view
        ]
        .into()
    }
}

fn limit_length(value: &str) -> &str {
    if value.len() <= config::CONFIG.interact.poster_text_len {
        value
    } else {
        &value[..config::CONFIG.interact.poster_text_len]
    }
}

/// This does not actually generates any messages
pub fn view_poster(poster: &becks_poster::Poster, limit: bool) -> Element<MainMessage> {
    let mut rows: Vec<Element<MainMessage>> = Vec::new();
    let mut current: Vec<Element<MainMessage>> = Vec::new();
    let mut count = 0;
    let mut rows_count = 0;
    for image in poster.images.iter() {
        count += 1;
        current.push(
            widget::image(image)
                .content_fit(iced::ContentFit::ScaleDown)
                .width(175)
                .into(),
        );
        if count == 3 {
            rows_count += 1;
            count = 0;
            rows.push(
                widget::Row::from_iter(std::mem::take(&mut current))
                    .spacing(5)
                    .into(),
            );
            if limit && rows_count >= config::CONFIG.interact.poster_image_len {
                break;
            }
        }
    }
    if !current.is_empty() {
        rows.push(widget::Row::from_iter(current).spacing(5).into());
    }
    let value = if limit {
        limit_length(&poster.value)
    } else {
        &poster.value
    };
    widget::column![
        widget::text(limit_length(value))
            .style(widget::text::base)
            .color(iced::Color::from_rgb8(255, 255, 255)),
        widget::Column::from_iter(rows).spacing(5)
    ]
    .padding(10)
    .spacing(10)
    .into()
}
