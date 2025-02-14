use crate::prelude::*;
use becks_crew::*;
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct CrewQueryPanel {
    by: Vec<CrewLocation>,
    crew: Option<crew_panel::CrewPanel>,
    error: bool,
    select_only: bool,
    allow_select_all: bool,
    selected: Arc<Mutex<HashSet<Id>>>,
}

#[derive(Debug, Clone)]
pub enum CrewQueryMessage {
    Add(CrewLocation),
    Update(usize, CrewLocation),
    Remove(usize),
    StartQuery,
    QueryDone(Acquire<crew::CrewList>),
    QueryErr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddChoice {
    Name,
    Gender,
    Social,
    Score,
    /// Dummy choice for a hint
    Select,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for AddChoice {
    fn to_string(&self) -> String {
        match self {
            Self::Name => assets::TEXT.get("crew_query_name"),
            Self::Gender => assets::TEXT.get("crew_query_gender"),
            Self::Social => assets::TEXT.get("crew_query_social"),
            Self::Score => assets::TEXT.get("crew_query_score"),
            Self::Select => assets::TEXT.get("crew_query_select"),
        }
        .to_owned()
    }
}

impl CrewQueryPanel {
    pub fn selection(&self) -> std::sync::MutexGuard<HashSet<Id>> {
        self.selected.lock().unwrap()
    }

    pub fn select_only(mut self) -> Self {
        self.select_only = true;
        self
    }

    pub fn allow_select_all(mut self) -> Self {
        self.allow_select_all = true;
        self
    }
}

impl Panel for CrewQueryPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        match message {
            MainMessage::CrewQueryMessage(message) => match message {
                CrewQueryMessage::Add(loc) => {
                    self.by.push(loc);
                    Task::none()
                }
                CrewQueryMessage::Update(index, loc) => {
                    if let Some(loc_ref) = self.by.get_mut(index) {
                        *loc_ref = loc;
                    }
                    Task::none()
                }
                CrewQueryMessage::Remove(index) => {
                    if index < self.by.len() {
                        self.by.remove(index);
                    }
                    Task::none()
                }
                CrewQueryMessage::StartQuery => {
                    let by = self.by.clone();
                    Task::perform(
                        async move { crew::CrewList::query(login.as_ref(), by).await },
                        |result| match result {
                            Ok(value) => MainMessage::CrewQueryMessage(
                                CrewQueryMessage::QueryDone(Acquire::new(value)),
                            ),
                            Err(err) => {
                                error!("When querying for crews, {}", err);
                                MainMessage::CrewQueryMessage(CrewQueryMessage::QueryErr)
                            }
                        },
                    )
                }
                CrewQueryMessage::QueryDone(list) => {
                    if let Some(list) = list.try_acquire() {
                        if self.select_only {
                            self.crew = Some(crew_panel::CrewPanel::new_with_select(
                                list,
                                self.selected.clone(),
                                self.allow_select_all,
                            ));
                        } else {
                            self.crew = Some(crew_panel::CrewPanel::new(list));
                        }
                        Task::done(MainMessage::CrewMessage(crew_panel::CrewMessage::Load))
                    } else {
                        Task::none()
                    }
                }
                CrewQueryMessage::QueryErr => {
                    self.error = true;
                    Task::none()
                }
            },
            _ => {
                if let Some(crew) = &mut self.crew {
                    crew.update_with_login(login, message)
                } else {
                    Task::none()
                }
            }
        }
    }
    fn view(&self) -> Element<MainMessage> {
        let mut column: Vec<Element<MainMessage>> = Vec::new();
        for (index, loc) in self.by.iter().enumerate() {
            column.push(view_location(index, loc))
        }
        widget::column![
            widget::text(if self.select_only {
                assets::TEXT.get("crew_query_title_select")
            } else {
                assets::TEXT.get("crew_query_title")
            }),
            widget::row![
                self.pick_add(),
                widget::button(if self.by.is_empty() {
                    assets::TEXT.get("crew_query_acquire")
                } else {
                    assets::TEXT.get("crew_query_start")
                })
                .style(widget::button::primary)
                .on_press(MainMessage::CrewQueryMessage(CrewQueryMessage::StartQuery))
            ]
            .spacing(10),
            widget::Column::from_iter(column),
        ]
        .push_maybe(self.crew.as_ref().map(|crew| {
            widget::scrollable(crew.view())
                .direction(widget::scrollable::Direction::Vertical(
                    widget::scrollable::Scrollbar::new(),
                ))
                .height(150)
        }))
        .push_maybe(if self.error {
            Some(widget::text(assets::TEXT.get("crew_query_error")).style(widget::text::danger))
        } else {
            None
        })
        .spacing(10)
        .extend(if self.select_only {
            vec![]
        } else {
            vec![
                widget::horizontal_rule(4).into(),
                widget::button(assets::TEXT.get("crew_query_create"))
                    .on_press(MainMessage::Open(Acquire::new(PanelHandle::new(
                        crew_create::CrewCreatePanel::default(),
                    ))))
                    .into(),
            ]
        })
        .into()
    }
    fn on_rewind_to(&mut self) -> Task<MainMessage> {
        self.crew = None;
        Task::none()
    }
}

impl CrewQueryPanel {
    fn pick_add(&self) -> Element<MainMessage> {
        widget::pick_list(
            [
                AddChoice::Name,
                AddChoice::Score,
                AddChoice::Social,
                AddChoice::Gender,
            ],
            Option::<&AddChoice>::Some(&AddChoice::Select),
            |choice| {
                MainMessage::CrewQueryMessage(CrewQueryMessage::Add(match choice {
                    AddChoice::Name => CrewLocation::Name(Default::default()),
                    AddChoice::Score => CrewLocation::Score(Score(i32::MIN)),
                    AddChoice::Gender => CrewLocation::Gender(Default::default()),
                    AddChoice::Social => CrewLocation::Social(Default::default()),
                    AddChoice::Select => CrewLocation::Name("You win!".to_owned()),
                }))
            },
        )
        .width(100)
        .into()
    }
}

fn view_location(index: usize, loc: &CrewLocation) -> Element<MainMessage> {
    use becks_crew::CrewLocation as Loc;
    let mut row: Vec<Element<MainMessage>> = Vec::new();
    match loc {
        Loc::Name(name) => row.extend([
            widget::text(assets::TEXT.get("crew_query_name")).into(),
            widget::text_input(assets::TEXT.get("crew_query_name_hint"), name)
                .on_input(move |value| {
                    MainMessage::CrewQueryMessage(CrewQueryMessage::Update(
                        index,
                        CrewLocation::Name(value),
                    ))
                })
                .into(),
        ]),
        Loc::Gender(gender) => row.extend([
            widget::text(assets::TEXT.get("crew_query_gender")).into(),
            widget::pick_list(Gender::all_repred(), Some(gender.repr()), move |gender| {
                let gender = Gender::unrepr(gender);
                MainMessage::CrewQueryMessage(CrewQueryMessage::Update(index, Loc::Gender(*gender)))
            })
            .into(),
        ]),
        Loc::Score(score) => row.extend([
            widget::text(assets::TEXT.get("crew_query_score")).into(),
            widget::text_input(
                assets::TEXT.get("crew_query_score_hint"),
                &if score.0 == i32::MIN {
                    String::new()
                } else {
                    score.0.to_string()
                },
            )
            .on_input(move |value| {
                if value.is_empty() {
                    MainMessage::CrewQueryMessage(CrewQueryMessage::Update(
                        index,
                        CrewLocation::Score(Score(i32::MIN)),
                    ))
                } else if let Ok(value) = value.parse() {
                    MainMessage::CrewQueryMessage(CrewQueryMessage::Update(
                        index,
                        CrewLocation::Score(Score(value)),
                    ))
                } else {
                    warn!("Unable to parse input: {}", value);
                    MainMessage::None
                }
            })
            .into(),
        ]),
        Loc::Social(social) => row.extend([
            widget::text(assets::TEXT.get("crew_query_social")).into(),
            widget::pick_list(Social::all_repred(), Some(social.repr()), move |social| {
                let social = Social::unrepr(social);
                MainMessage::CrewQueryMessage(CrewQueryMessage::Update(index, Loc::Social(*social)))
            })
            .into(),
        ]),
        _ => {
            error!("Unknown query location: {:?}", loc);
            row.push(
                widget::text(assets::TEXT.get("unexpected"))
                    .style(widget::text::danger)
                    .into(),
            );
        }
    };
    row.push(
        widget::button(widget::image("assets/remove.png").content_fit(iced::ContentFit::ScaleDown))
            .on_press(MainMessage::CrewQueryMessage(CrewQueryMessage::Remove(
                index,
            )))
            .into(),
    );
    widget::Row::from_iter(row).height(30).into()
}
