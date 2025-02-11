use crate::prelude::*;

#[derive(Debug)]
pub struct CrewPanel {
    crew: Arc<crew::CrewList>,
    loaded: Vec<becks_crew::CrewData>,
    is_loaded: bool,
}

#[derive(Debug, Clone)]
pub enum CrewMessage {
    Reload,
    Load,
    Loaded(Acquire<Vec<becks_crew::CrewData>>),
}

impl CrewPanel {
    pub fn new(crew: crew::CrewList) -> Self {
        Self {
            crew: Arc::new(crew),
            loaded: Default::default(),
            is_loaded: false,
        }
    }
}

impl Panel for CrewPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::CrewMessage(message) => match message {
                CrewMessage::Load => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move {
                            let mut crew_list = Vec::new();
                            for crew in crew.iter() {
                                crew_list
                                    .push(crew.write().await.load(login.as_ref()).await.cloned())
                            }
                            crew_list
                        },
                        |crew_list| {
                            let crew_list = crew_list.into_iter().collect::<Result<Vec<_>>>();
                            match crew_list {
                                Ok(crew_list) => MainMessage::CrewMessage(CrewMessage::Loaded(
                                    Acquire::new(crew_list),
                                )),
                                Err(err) => {
                                    error!("When loading crew list, {}", err);
                                    MainMessage::None
                                }
                            }
                        },
                    )
                }
                CrewMessage::Reload => {
                    self.loaded.clear();
                    self.is_loaded = false;
                    let crew = self.crew.clone();
                    Task::perform(async move { crew.reload(login.as_ref()).await }, |result| {
                        if let Err(err) = result {
                            error!("When reloading crew, {}", err);
                        }
                        MainMessage::CrewMessage(CrewMessage::Load)
                    })
                }
                CrewMessage::Loaded(loaded) => {
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
        let crew_view: Element<MainMessage> = if self.is_loaded {
            if self.crew.is_empty() {
                widget::text(assets::TEXT.get("crew_empty")).into()
            } else {
                let mut column: Vec<Element<MainMessage>> = Vec::new();
                for crew in self.loaded.iter() {
                    column.push(view_crew(crew));
                    column.push(widget::Rule::horizontal(25).into());
                }
                widget::scrollable(widget::Column::from_iter(column)).into()
            }
        } else {
            widget::text(assets::TEXT.get("crew_loading"))
                .style(widget::text::secondary)
                .into()
        };
        widget::column![
            widget::text(assets::TEXT.get("crew_title")).style(widget::text::primary),
            crew_view
        ]
        .into()
    }
}

fn view_crew(crew: &becks_crew::CrewData) -> Element<MainMessage> {
    widget::container(widget::row![
        widget::text(&crew.name).align_y(iced::Alignment::Center) // TODO: Show more information
    ])
    .style(widget::container::rounded_box)
    .into()
}
