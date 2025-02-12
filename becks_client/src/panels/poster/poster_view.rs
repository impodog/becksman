use crate::prelude::*;
use becks_poster::Poster;
use poster_panel::view_poster;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PosterViewPanel {
    poster: Arc<Mutex<poster::PosterInfo>>,
    data: Option<Poster>,
    error: bool,
}

#[derive(Debug, Clone)]
pub enum PosterViewMessage {
    Load,
    Loaded(Acquire<Poster>),
    LoadError,
}

impl PosterViewPanel {
    pub fn new(poster: poster::PosterInfo) -> Self {
        Self {
            poster: Arc::new(Mutex::new(poster)),
            data: None,
            error: false,
        }
    }
}

impl Panel for PosterViewPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = true;
        match message {
            MainMessage::PosterViewMessage(message) => match message {
                PosterViewMessage::Load => {
                    let poster = self.poster.clone();
                    Task::perform(
                        async move { poster.lock().await.load(login.as_ref()).await.cloned() },
                        |data| match data {
                            Ok(data) => MainMessage::PosterViewMessage(PosterViewMessage::Loaded(
                                Acquire::new(data),
                            )),
                            Err(err) => {
                                warn!("When loading poster, {}", err);
                                MainMessage::PosterViewMessage(PosterViewMessage::LoadError)
                            }
                        },
                    )
                }
                PosterViewMessage::Loaded(data) => {
                    if let Some(data) = data.try_acquire() {
                        self.data = Some(data);
                    }
                    Task::none()
                }
                PosterViewMessage::LoadError => {
                    self.error = true;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<MainMessage> {
        if let Some(data) = self.data.as_ref() {
            widget::container(widget::scrollable(view_poster(data, false)).height(300))
                .style(widget::container::rounded_box)
                .into()
        } else {
            widget::text(assets::TEXT.get("poster_view_loading"))
                .style(widget::text::secondary)
                .into()
        }
    }

    fn on_start_up(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::PosterViewMessage(PosterViewMessage::Load))
    }
}
