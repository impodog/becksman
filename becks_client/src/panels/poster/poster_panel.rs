use crate::prelude::*;

#[derive(Debug, Default)]
pub struct PosterPanel {
    poster: Arc<poster::PosterList>,
    loaded: Vec<becks_poster::Poster>,
    is_loaded: bool,
}

impl PosterPanel {
    pub fn new(poster: poster::PosterList) -> Self {
        Self {
            poster: Arc::new(poster),
            loaded: Default::default(),
            is_loaded: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PosterMessage {
    Reload,
    Load,
    Loaded(Acquire<Vec<becks_poster::Poster>>),
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
            },
            _ => Task::none(),
        }
    }
    fn view(&self) -> Element<MainMessage> {
        let poster_view: Element<MainMessage> = if self.is_loaded {
            if self.poster.is_empty() {
                widget::text(assets::TEXT.get("poster_empty")).into()
            } else {
                let mut column: Vec<Element<MainMessage>> = Vec::new();
                for poster in self.loaded.iter() {
                    column.push(view_poster(poster));
                    column.push(widget::Rule::horizontal(25).into());
                }
                widget::scrollable(widget::Column::from_iter(column)).into()
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

fn view_poster(poster: &becks_poster::Poster) -> Element<MainMessage> {
    let mut rows: Vec<Element<MainMessage>> = Vec::new();
    let mut current: Vec<Element<MainMessage>> = Vec::new();
    let mut count = 0;
    for image in poster.images.iter() {
        count += 1;
        current.push(
            widget::image(image)
                .content_fit(iced::ContentFit::Contain)
                .into(),
        );
        if count == 3 {
            count = 0;
            rows.push(widget::Row::from_iter(std::mem::take(&mut current)).into());
        }
    }
    widget::column![
        widget::text(&poster.value)
            .style(widget::text::base)
            .color(iced::Color::from_rgb8(255, 255, 255)),
        widget::Column::from_iter(rows)
    ]
    .into()
}
