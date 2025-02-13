use crate::prelude::*;
use becks_crew::*;
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug)]
pub struct CrewPanel {
    crew: Arc<crew::CrewList>,
    loaded: Vec<(Id, CrewData)>,
    is_loaded: bool,
    select_only: bool,
    allow_select_all: bool,
    pub selected: Arc<Mutex<HashSet<Id>>>,
}

#[derive(Debug, Clone)]
pub enum CrewMessage {
    Reload,
    Load,
    Loaded(Acquire<Vec<(Id, CrewData)>>),
    Select(Id),
    SelectAll,
}

impl CrewPanel {
    pub fn new(crew: crew::CrewList) -> Self {
        Self {
            crew: Arc::new(crew),
            loaded: Default::default(),
            is_loaded: false,
            select_only: false,
            allow_select_all: false,
            selected: Default::default(),
        }
    }

    pub fn new_with_select(
        crew: crew::CrewList,
        selected: Arc<Mutex<HashSet<Id>>>,
        allow_select_all: bool,
    ) -> Self {
        Self {
            crew: Arc::new(crew),
            loaded: Default::default(),
            is_loaded: false,
            select_only: true,
            allow_select_all,
            selected,
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
                                let id = crew.read().await.id();
                                crew_list.push(
                                    crew.write()
                                        .await
                                        .load(login.as_ref())
                                        .await
                                        .cloned()
                                        .map(|data| (id, data)),
                                )
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
                CrewMessage::Select(id) => {
                    let mut selected = self.selected.lock().unwrap();
                    if !selected.insert(id) {
                        selected.remove(&id);
                    }
                    Task::none()
                }
                CrewMessage::SelectAll => {
                    let mut selected = self.selected.lock().unwrap();
                    if selected.len() == self.loaded.len() {
                        selected.clear();
                    } else {
                        *selected = HashSet::from_iter(self.loaded.iter().map(|(id, _)| *id));
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
                for (id, crew) in self.loaded.iter() {
                    column.push(view_crew(
                        *id,
                        crew,
                        &self.selected.lock().unwrap(),
                        self.select_only,
                    ));
                    column.push(widget::Rule::horizontal(1).into());
                }
                if self.select_only && self.allow_select_all {
                    column.push(
                        widget::button(assets::TEXT.get("crew_select_all"))
                            .height(30)
                            .on_press(MainMessage::CrewMessage(CrewMessage::SelectAll))
                            .into(),
                    );
                }
                widget::Column::from_iter(column).into()
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
        .padding(10)
        .into()
    }
    fn on_rewind_to(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::CrewMessage(CrewMessage::Reload))
    }
}

fn view_crew<'a>(
    id: Id,
    crew: &'a CrewData,
    selected: &HashSet<Id>,
    select_only: bool,
) -> Element<'a, MainMessage> {
    let button = if select_only {
        widget::button(if selected.contains(&id) {
            widget::image("assets/remove.png")
        } else {
            widget::image("assets/add.png")
        })
        .height(30)
        .width(30)
        .on_press(MainMessage::CrewMessage(CrewMessage::Select(id)))
    } else {
        widget::button(widget::image("assets/jump.png"))
            .height(30)
            .width(30)
            .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                crew_info::CrewInfoPanel::new(id),
            ))))
    };
    widget::container(widget::row![
        widget::text(&crew.name).align_y(iced::Alignment::Center),
        widget::horizontal_space(),
        button,
    ])
    .style(widget::container::rounded_box)
    .into()
}
