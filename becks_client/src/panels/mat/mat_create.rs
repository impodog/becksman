use crate::prelude::*;
use becks_match::*;

#[derive(Debug)]
pub struct MatCreatePanel {
    selection: Option<crew_query::CrewQueryPanel>,
    select_is_left: bool,
    total: usize,
    left: Option<Id>,
    right: Option<Id>,
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

impl Panel for MatCreatePanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        self.local_error = false;
        match message {
            MainMessage::MatCreateMessage(message) => match message {
                MatCreateMessage::StartCreate => {
                    if let Some((left, right, rounds)) = self
                        .left
                        .and_then(|left| self.right.map(move |right| (left, right)))
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
                    if let Some(id) = selection.iter().next().copied() {
                        std::mem::drop(selection);
                        self.selection = None;
                        if self.select_is_left {
                            self.left = Some(id);
                        } else {
                            self.right = Some(id);
                        }
                    }
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
                    widget::button(assets::TEXT.get("mat_create_left")).on_press(
                        MainMessage::MatCreateMessage(MatCreateMessage::StartSelect(true))
                    ),
                    widget::button(assets::TEXT.get("mat_create_right")).on_press(
                        MainMessage::MatCreateMessage(MatCreateMessage::StartSelect(false))
                    ),
                ]
                .spacing(10)
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
