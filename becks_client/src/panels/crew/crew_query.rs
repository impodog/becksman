use crate::prelude::*;
use becks_crew::*;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct QueryCrewPanel {
    by: Vec<CrewLocation>,
    crew: Option<crew_panel::CrewPanel>,
    selected: Vec<Id>,
    error: bool,
}

#[derive(Debug, Clone)]
pub enum QueryCrewMessage {
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
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for AddChoice {
    fn to_string(&self) -> String {
        match self {
            Self::Name => assets::TEXT.get("crew_query_name"),
            Self::Gender => assets::TEXT.get("crew_query_gender"),
            Self::Social => assets::TEXT.get("crew_query_social"),
            Self::Score => assets::TEXT.get("crew_query_score"),
        }
        .to_owned()
    }
}

impl Panel for QueryCrewPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::QueryCrewMessage(message) => match message {
                QueryCrewMessage::Add(loc) => {
                    self.by.push(loc);
                    Task::none()
                }
                QueryCrewMessage::Update(index, loc) => {
                    if let Some(loc_ref) = self.by.get_mut(index) {
                        *loc_ref = loc;
                    }
                    Task::none()
                }
                QueryCrewMessage::Remove(index) => {
                    if index < self.by.len() {
                        self.by.remove(index);
                    }
                    Task::none()
                }
                QueryCrewMessage::StartQuery => {
                    let by = self.by.clone();
                    Task::perform(
                        async move { crew::CrewList::query(login.as_ref(), by).await },
                        |result| match result {
                            Ok(value) => MainMessage::QueryCrewMessage(
                                QueryCrewMessage::QueryDone(Acquire::new(value)),
                            ),
                            Err(err) => {
                                error!("When querying for crews, {}", err);
                                MainMessage::QueryCrewMessage(QueryCrewMessage::QueryErr)
                            }
                        },
                    )
                }
                QueryCrewMessage::QueryDone(list) => {
                    if let Some(list) = list.try_acquire() {
                        self.crew = Some(crew_panel::CrewPanel::new(list));
                        self.error = false;
                        Task::done(MainMessage::CrewMessage(crew_panel::CrewMessage::Load))
                    } else {
                        Task::none()
                    }
                }
                QueryCrewMessage::QueryErr => {
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
            widget::text(assets::TEXT.get("crew_query_title")).align_x(iced::Alignment::Center),
            widget::row![
                self.pick_add(),
                widget::button(assets::TEXT.get("crew_query_start"))
                    .style(widget::button::primary)
                    .on_press(MainMessage::QueryCrewMessage(QueryCrewMessage::StartQuery))
            ],
            widget::horizontal_rule(2),
            widget::Column::from_iter(column),
            widget::horizontal_rule(2),
        ]
        .push_maybe(self.crew.as_ref().map(|crew| crew.view()))
        .push_maybe(if self.error {
            Some(widget::text(assets::TEXT.get("crew_query_error")).style(widget::text::danger))
        } else {
            None
        })
        .into()
    }
}

impl QueryCrewPanel {
    fn pick_add(&self) -> Element<MainMessage> {
        widget::pick_list(
            [
                AddChoice::Name,
                AddChoice::Score,
                AddChoice::Social,
                AddChoice::Gender,
            ],
            Option::<&AddChoice>::None,
            |choice| {
                MainMessage::QueryCrewMessage(QueryCrewMessage::Add(match choice {
                    AddChoice::Name => CrewLocation::Name(Default::default()),
                    AddChoice::Score => CrewLocation::Score(Score(i32::MIN)),
                    AddChoice::Gender => CrewLocation::Gender(Default::default()),
                    AddChoice::Social => CrewLocation::Social(Default::default()),
                }))
            },
        )
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
                    MainMessage::QueryCrewMessage(QueryCrewMessage::Update(
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
                MainMessage::QueryCrewMessage(QueryCrewMessage::Update(index, Loc::Gender(*gender)))
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
                    MainMessage::QueryCrewMessage(QueryCrewMessage::Update(
                        index,
                        CrewLocation::Score(Score(i32::MIN)),
                    ))
                } else if let Ok(value) = value.parse() {
                    MainMessage::QueryCrewMessage(QueryCrewMessage::Update(
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
                MainMessage::QueryCrewMessage(QueryCrewMessage::Update(index, Loc::Social(*social)))
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
            .on_press(MainMessage::QueryCrewMessage(QueryCrewMessage::Remove(
                index,
            )))
            .into(),
    );
    widget::Row::from_iter(row).height(30).into()
}
