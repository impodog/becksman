use crate::arrange::*;
use crate::prelude::*;
use becks_match::*;
use mat_panel::MatMessage;
use std::sync::Mutex;

#[derive(Default, Debug)]
pub struct MatArrangePanel {
    selection: Option<crew_query::CrewQueryPanel>,
    selected: Option<Vec<Id>>,
    arranger: Option<Mutex<Arranger>>,
    names: Vec<Vec<String>>,
    group_size: usize,
    current_group: usize,
    error: bool,
    local_error: bool,
}

#[derive(Debug, Clone)]
pub enum MatArrangeMessage {
    StartArrange,
    ArrangeAcquired(Acquire<Arranger>),
    NamesAcquired(Acquire<Vec<Vec<String>>>),
    StartSelection,
    EndSelection,
    UpdateGroupSize(usize),
    NextGroup,
    PrevGroup,
    Error,
    LocalError,
}

impl Panel for MatArrangePanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        self.local_error = false;
        match message {
            MainMessage::MatArrangeMessage(message) => match message {
                MatArrangeMessage::StartArrange => {
                    if let Some(selected) = std::mem::take(&mut self.selected) {
                        let group_size = if self.group_size == 0 {
                            3
                        } else {
                            self.group_size
                        };
                        Task::perform(
                            async move {
                                let mut items = Vec::new();
                                for id in selected.into_iter() {
                                    let mut info = crew::CrewInfo::new(id);
                                    let data = info.load(login.as_ref()).await?;
                                    items.push(ArrangeItem {
                                        id,
                                        score: data.score,
                                    });
                                }
                                Result::<Arranger>::Ok(Arranger::new(items, group_size))
                            },
                            |result| match result {
                                Ok(arranger) => MainMessage::MatArrangeMessage(
                                    MatArrangeMessage::ArrangeAcquired(Acquire::new(arranger)),
                                ),
                                Err(err) => {
                                    error!("When acquiring arranger crew, {}", err);
                                    MainMessage::MatArrangeMessage(MatArrangeMessage::Error)
                                }
                            },
                        )
                    } else {
                        self.current_group = 0;
                        Task::done(MainMessage::MatArrangeMessage(
                            MatArrangeMessage::LocalError,
                        ))
                    }
                }
                MatArrangeMessage::ArrangeAcquired(arranger) => {
                    if let Some(mut arranger) = arranger.try_acquire() {
                        arranger.arrange();
                        let groups = arranger.groups.clone();
                        self.arranger = Some(Mutex::new(arranger));
                        Task::perform(
                            async move {
                                let mut result = Vec::new();
                                for group in groups.iter() {
                                    let mut group_names = Vec::new();
                                    for id in group.all.iter().copied() {
                                        group_names.push(std::mem::take(
                                            &mut crew::CrewInfo::new(id)
                                                .load(login.as_ref())
                                                .await?
                                                .name,
                                        ));
                                    }
                                    result.push(group_names);
                                }
                                Result::<_>::Ok(result)
                            },
                            |result| match result {
                                Ok(names) => MainMessage::MatArrangeMessage(
                                    MatArrangeMessage::NamesAcquired(Acquire::new(names)),
                                ),
                                Err(err) => {
                                    error!("When acquiring names, {}", err);
                                    MainMessage::MatArrangeMessage(MatArrangeMessage::Error)
                                }
                            },
                        )
                    } else {
                        Task::none()
                    }
                }
                MatArrangeMessage::NamesAcquired(names) => {
                    if let Some(names) = names.try_acquire() {
                        self.names = names;
                    }
                    Task::none()
                }
                MatArrangeMessage::StartSelection => {
                    self.selection = Some(
                        crew_query::CrewQueryPanel::default()
                            .select_only()
                            .allow_select_all(),
                    );
                    Task::none()
                }
                MatArrangeMessage::EndSelection => {
                    if let Some(selection) = self.selection.take() {
                        self.selected = Some(selection.selection().iter().copied().collect());
                    }
                    Task::none()
                }
                MatArrangeMessage::UpdateGroupSize(group_size) => {
                    self.group_size = group_size;
                    Task::none()
                }
                MatArrangeMessage::NextGroup => {
                    self.current_group = (self.current_group + 1).min(
                        self.arranger
                            .as_ref()
                            .map(|arranger| arranger.lock().unwrap().groups.len().saturating_sub(1))
                            .unwrap_or_default(),
                    );
                    Task::none()
                }
                MatArrangeMessage::PrevGroup => {
                    self.current_group = self.current_group.saturating_sub(1);
                    Task::none()
                }
                MatArrangeMessage::Error => {
                    self.error = true;
                    Task::none()
                }
                MatArrangeMessage::LocalError => {
                    self.local_error = true;
                    Task::none()
                }
            },
            _ => {
                if let Some(selection) = self.selection.as_mut() {
                    selection.update_with_login(login, message)
                } else {
                    Task::none()
                }
            }
        }
    }
    fn view(&self) -> Element<MainMessage> {
        let mut sub_column: Vec<Element<MainMessage>> = Vec::new();
        if let Some(arranger) = self.arranger.as_ref() {
            let arranger = arranger.lock().unwrap();
            sub_column.push(view_arranger(
                &arranger,
                self.names.as_slice(),
                self.current_group,
            ));
        } else if let Some(selection) = self.selection.as_ref() {
            sub_column.push(
                widget::button(assets::TEXT.get("mat_arrange_select_end"))
                    .style(widget::button::primary)
                    .on_press(MainMessage::MatArrangeMessage(
                        MatArrangeMessage::EndSelection,
                    ))
                    .into(),
            );
            sub_column.push(
                widget::container(selection.view())
                    .width(iced::Fill)
                    .style(widget::container::rounded_box)
                    .into(),
            );
        } else {
            sub_column.push(
                widget::row![widget::text_input(
                    assets::TEXT.get("mat_arrange_groupsize"),
                    &if self.group_size == 0 {
                        "".to_owned()
                    } else {
                        self.group_size.to_string()
                    },
                )
                .on_input(|value| {
                    MainMessage::MatArrangeMessage(MatArrangeMessage::UpdateGroupSize(
                        if value.is_empty() {
                            0
                        } else {
                            value.parse().unwrap_or(self.group_size)
                        },
                    ))
                }),]
                .push_maybe(self.selected.as_ref().map(|selection| {
                    widget::text(format!(
                        "{}: {}",
                        assets::TEXT.get("mat_arrange_total"),
                        selection.len(),
                    ))
                }))
                .spacing(20)
                .into(),
            );
            sub_column.push(
                widget::button(assets::TEXT.get("mat_arrange_select"))
                    .style(widget::button::primary)
                    .on_press(MainMessage::MatArrangeMessage(
                        MatArrangeMessage::StartSelection,
                    ))
                    .into(),
            );
            sub_column.push(
                widget::button(assets::TEXT.get("mat_arrange_arrange"))
                    .style(widget::button::primary)
                    .on_press_maybe(
                        if self
                            .selected
                            .as_ref()
                            .is_some_and(|selected| !selected.is_empty())
                            && self.group_size != 0
                        {
                            Some(MainMessage::MatArrangeMessage(
                                MatArrangeMessage::StartArrange,
                            ))
                        } else {
                            None
                        },
                    )
                    .into(),
            );
        }
        if self.error {
            sub_column.push(
                widget::text(assets::TEXT.get("mat_arrange_error"))
                    .style(widget::text::danger)
                    .into(),
            );
        }
        if self.local_error {
            sub_column.push(
                widget::text(assets::TEXT.get("mat_arrange_localerror"))
                    .style(widget::text::danger)
                    .into(),
            );
        }
        let sub_column = widget::Column::from_iter(sub_column)
            .spacing(10)
            .padding(10);
        widget::column![
            widget::text(assets::TEXT.get("mat_arrange_title")),
            sub_column,
        ]
        .into()
    }
}

fn view_group<'n>(group: &Group, names: &'n [String]) -> Element<'n, MainMessage> {
    use iced_aw::{grid, grid_row, Grid};
    let mut rows = Vec::new();
    let mut base_row: Vec<Element<MainMessage>> = Vec::new();
    base_row.push(widget::horizontal_space().into());
    for (_id, name) in group.all.iter().copied().zip(names.iter()) {
        base_row.push(widget::text(name).into());
    }
    rows.push(grid_row(base_row));
    for (left_id, left_name) in group.all.iter().copied().zip(names.iter()) {
        let mut row: Vec<Element<MainMessage>> = Vec::new();
        row.push(widget::text(left_name).into());
        for (right_id, _right_name) in group.all.iter().copied().zip(names.iter()) {
            if right_id == left_id {
                row.push(widget::horizontal_space().into());
            } else {
                row.push(
                    widget::button(assets::TEXT.get("mat_arrange_mat_create"))
                        .width(iced::Fill)
                        .on_press_with(move || {
                            MainMessage::Open(Acquire::new(PanelHandle::new(
                                mat_create::MatCreatePanel::default()
                                    .with_left(left_id)
                                    .with_right(right_id),
                            )))
                        })
                        .into(),
                );
            }
        }
        let row = grid_row(row);
        rows.push(row);
    }
    grid(rows).row_spacing(5).column_spacing(5).into()
}

fn view_arranger<'n>(
    arranger: &Arranger,
    names: &'n [Vec<String>],
    current_group: usize,
) -> Element<'n, MainMessage> {
    widget::column![widget::row![
        widget::text(format!("{} / {}", current_group + 1, arranger.groups.len())),
        widget::button(assets::TEXT.get("mat_arrange_prev"))
            .on_press(MainMessage::MatArrangeMessage(MatArrangeMessage::PrevGroup)),
        widget::button(assets::TEXT.get("mat_arrange_next"))
            .on_press(MainMessage::MatArrangeMessage(MatArrangeMessage::NextGroup))
    ]
    .spacing(20),]
    .push_maybe(
        if let Some((group, names)) = arranger.groups.get(current_group).and_then(|group| {
            names
                .get(current_group)
                .map(move |names| (group, names.as_slice()))
        }) {
            Some(widget::scrollable(view_group(group, names)).direction(
                widget::scrollable::Direction::Both {
                    vertical: widget::scrollable::Scrollbar::new(),
                    horizontal: widget::scrollable::Scrollbar::new(),
                },
            ))
        } else {
            None
        },
    )
    .into()
}
