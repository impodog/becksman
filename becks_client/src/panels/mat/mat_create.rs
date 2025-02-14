use crate::prelude::*;
use becks_match::*;

#[derive(Debug, Clone)]
struct Selection {
    id: Id,
    name: Option<String>,
}

#[derive(Debug)]
pub struct MatCreatePanel {
    selection: Option<crew_query::CrewQueryPanel>,
    select_is_left: bool,
    total: usize,
    left: Option<Selection>,
    right: Option<Selection>,
    rounds: Vec<Option<Round>>,
    quit: Quit,
    notes: String,
    error: bool,
    local_error: bool,
}

#[derive(Debug, Clone)]
pub enum MatCreateMessage {
    StartCreate,
    Error,
    LocalError,
    Created,
    UpdateTotal(usize),
    StartSelect(bool),
    StartGetName(bool),
    NameAcquired(bool, String),
    // These are not required since modification could be directly done
    // UpdateLeft(Id),
    // UpdateRight(Id),
    ModifyRound(usize, bool),
    UpdateQuit(Quit),
    UpdateNotes(String),
}

impl Default for MatCreatePanel {
    fn default() -> Self {
        let total = config::CONFIG.interact.default_rounds;
        Self {
            selection: None,
            select_is_left: true,
            total,
            left: None,
            right: None,
            rounds: vec![None; total],
            quit: Default::default(),
            notes: Default::default(),
            error: false,
            local_error: false,
        }
    }
}

impl MatCreatePanel {
    pub fn with_left(mut self, id: Id) -> Self {
        self.left = Some(Selection { id, name: None });
        self
    }

    pub fn with_right(mut self, id: Id) -> Self {
        self.right = Some(Selection { id, name: None });
        self
    }
}

impl Panel for MatCreatePanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        self.local_error = false;
        match message {
            MainMessage::MatCreateMessage(message) => match message {
                MatCreateMessage::StartCreate => {
                    if let Some((left, right, rounds)) = self
                        .left
                        .as_ref()
                        .and_then(|left| self.right.as_ref().map(move |right| (left.id, right.id)))
                        .and_then(|(left, right)| {
                            let rounds = self.rounds.iter().copied().collect::<Option<Vec<_>>>();
                            rounds.map(move |round| (left, right, round))
                        })
                    {
                        let mut mat = Match::new(self.total, left, right, current_timestamp());
                        mat.rounds = rounds;
                        mat.quit = self.quit;
                        mat.notes = self.notes.clone();
                        Task::perform(
                            async move { mat::MatchInfo::create(login.as_ref(), mat).await },
                            |result| match result {
                                Ok(_) => MainMessage::MatCreateMessage(MatCreateMessage::Created),
                                Err(err) => {
                                    error!("When creating match, {}", err);
                                    MainMessage::MatCreateMessage(MatCreateMessage::Error)
                                }
                            },
                        )
                    } else {
                        Task::done(MainMessage::MatCreateMessage(MatCreateMessage::LocalError))
                    }
                }
                MatCreateMessage::StartSelect(left) => {
                    self.select_is_left = left;
                    self.selection = Some(crew_query::CrewQueryPanel::default().select_only());
                    Task::none()
                }
                MatCreateMessage::StartGetName(left) => {
                    let id = if left {
                        self.left.as_ref().map(|left| left.id)
                    } else {
                        self.right.as_ref().map(|right| right.id)
                    };
                    if let Some(id) = id {
                        Task::perform(
                            async move {
                                crew::CrewInfo::new(id)
                                    .load(login.as_ref())
                                    .await
                                    .map(|data| std::mem::take(&mut data.name))
                            },
                            move |result| match result {
                                Ok(name) => MainMessage::MatCreateMessage(
                                    MatCreateMessage::NameAcquired(left, name),
                                ),
                                Err(err) => {
                                    error!("When acquiring match crew name, {}", err);
                                    MainMessage::None
                                }
                            },
                        )
                    } else {
                        Task::none()
                    }
                }
                MatCreateMessage::NameAcquired(left, name) => {
                    #[allow(clippy::collapsible_else_if)]
                    if left {
                        if let Some(left) = self.left.as_mut() {
                            left.name = Some(name);
                        } else {
                            warn!("When acquired name {}, left does not exist", name);
                        }
                    } else {
                        if let Some(right) = self.right.as_mut() {
                            right.name = Some(name);
                        } else {
                            warn!("When acquired name {}, right does not exist", name);
                        }
                    }
                    Task::none()
                }
                MatCreateMessage::LocalError => {
                    self.local_error = true;
                    Task::none()
                }
                MatCreateMessage::Error => {
                    self.error = true;
                    Task::none()
                }
                MatCreateMessage::Created => Task::done(MainMessage::Rewind),
                MatCreateMessage::UpdateTotal(len) => {
                    self.total = len;
                    self.rounds.resize(len, None);
                    Task::none()
                }
                MatCreateMessage::ModifyRound(index, left_win) => {
                    if let Some(round) = self.rounds.get_mut(index) {
                        *round = Some(Round { left_win });
                    }
                    Task::none()
                }
                MatCreateMessage::UpdateQuit(quit) => {
                    self.quit = quit;
                    Task::none()
                }
                MatCreateMessage::UpdateNotes(notes) => {
                    self.notes = notes;
                    Task::none()
                }
            },
            _ => {
                if let Some(selection) = self.selection.as_mut() {
                    let task = selection.update_with_login(login, message);
                    let selection = selection.selection();
                    let task = if let Some(id) = selection.iter().next().copied() {
                        std::mem::drop(selection);
                        self.selection = None;
                        if self.select_is_left {
                            self.left = Some(Selection { id, name: None });
                            task.chain(Task::done(MainMessage::MatCreateMessage(
                                MatCreateMessage::StartGetName(true),
                            )))
                        } else {
                            self.right = Some(Selection { id, name: None });
                            task.chain(Task::done(MainMessage::MatCreateMessage(
                                MatCreateMessage::StartGetName(false),
                            )))
                        }
                    } else {
                        task
                    };
                    task
                } else {
                    Task::none()
                }
            }
        }
    }
    fn view(&self) -> Element<MainMessage> {
        let sub_column = {
            let mut column: Vec<Element<MainMessage>> = Vec::new();
            column.push(
                widget::button(assets::TEXT.get("mat_create_create"))
                    .style(widget::button::primary)
                    .on_press(MainMessage::MatCreateMessage(MatCreateMessage::StartCreate))
                    .into(),
            );
            column.push(widget::horizontal_rule(2).into());
            column.push(
                widget::row![
                    widget::button(assets::TEXT.get("mat_create_left"))
                        .on_press(MainMessage::MatCreateMessage(
                            MatCreateMessage::StartSelect(true)
                        ))
                        .style(if self.left.is_some() {
                            widget::button::secondary
                        } else {
                            widget::button::primary
                        }),
                    widget::button(assets::TEXT.get("mat_create_right"))
                        .on_press(MainMessage::MatCreateMessage(
                            MatCreateMessage::StartSelect(false)
                        ))
                        .style(if self.right.is_some() {
                            widget::button::secondary
                        } else {
                            widget::button::primary
                        }),
                ]
                .spacing(10)
                .into(),
            );
            column.push(
                widget::text(format!(
                    "{} {} {}",
                    self.left
                        .as_ref()
                        .and_then(|selection| selection.name.as_deref())
                        .unwrap_or_else(|| assets::TEXT.get("mat_create_left_pending")),
                    assets::TEXT.get("vs"),
                    self.right
                        .as_ref()
                        .and_then(|selection| selection.name.as_deref())
                        .unwrap_or_else(|| assets::TEXT.get("mat_create_right_pending")),
                ))
                .into(),
            );
            if let Some(selection) = self.selection.as_ref() {
                column.push(
                    widget::container(
                        widget::scrollable(selection.view())
                            .height(iced::FillPortion(1000))
                            .width(iced::Fill),
                    )
                    .height(iced::FillPortion(1000))
                    .style(widget::container::rounded_box)
                    .into(),
                );
            } else {
                column.push(view_total(self.total));
                column.push(
                    widget::scrollable(view_rounds(self.rounds.as_slice()))
                        .direction(widget::scrollable::Direction::Horizontal(
                            widget::scrollable::Scrollbar::new(),
                        ))
                        .height(100)
                        .into(),
                );
            }
            if self.error {
                column.push(
                    widget::text(assets::TEXT.get("mat_create_error"))
                        .style(widget::text::danger)
                        .into(),
                )
            }
            if self.local_error {
                column.push(
                    widget::text(assets::TEXT.get("mat_create_localerror"))
                        .style(widget::text::danger)
                        .into(),
                )
            }
            widget::Column::from_iter(column).spacing(10).padding(20)
        };

        widget::column![
            widget::text(assets::TEXT.get("mat_create_title")),
            sub_column
        ]
        .into()
    }

    fn on_start_up(&mut self) -> Task<MainMessage> {
        let left_task = if self.left.is_some() {
            Task::done(MainMessage::MatCreateMessage(
                MatCreateMessage::StartGetName(true),
            ))
        } else {
            Task::none()
        };
        let right_task = if self.right.is_some() {
            Task::done(MainMessage::MatCreateMessage(
                MatCreateMessage::StartGetName(false),
            ))
        } else {
            Task::none()
        };
        left_task.chain(right_task)
    }
}

fn view_total(total: usize) -> Element<'static, MainMessage> {
    widget::row![
        widget::text(assets::TEXT.get("mat_create_total")),
        widget::horizontal_space(),
        widget::text(total.to_string()),
        widget::button("+")
            .style(widget::button::secondary)
            .width(30)
            .height(30)
            .on_press(MainMessage::MatCreateMessage(
                MatCreateMessage::UpdateTotal(total + 1)
            )),
        widget::button("-")
            .style(widget::button::secondary)
            .width(30)
            .height(30)
            .on_press(MainMessage::MatCreateMessage(
                MatCreateMessage::UpdateTotal(if total > 1 { total - 1 } else { 1 })
            ))
    ]
    .into()
}

fn view_rounds(rounds: &[Option<Round>]) -> Element<MainMessage> {
    let mut row: Vec<Element<MainMessage>> = Vec::new();
    row.push(widget::text(assets::TEXT.get("mat_create_rounds")).into());
    for (index, round) in rounds.iter().enumerate() {
        row.push(
            widget::row![
                widget::text((index + 1).to_string()),
                widget::column![
                    widget::radio(
                        assets::TEXT.get("mat_create_left_win"),
                        true,
                        round.map(|round| round.left_win),
                        |value| MainMessage::MatCreateMessage(MatCreateMessage::ModifyRound(
                            index, value
                        ))
                    ),
                    widget::radio(
                        assets::TEXT.get("mat_create_right_win"),
                        false,
                        round.map(|round| round.left_win),
                        |value| MainMessage::MatCreateMessage(MatCreateMessage::ModifyRound(
                            index, value
                        ))
                    )
                ]
                .height(60)
                .spacing(10)
            ]
            .spacing(3)
            .into(),
        );
    }
    widget::Row::from_iter(row).spacing(8).into()
}
