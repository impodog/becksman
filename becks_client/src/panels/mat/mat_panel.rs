use crate::prelude::*;
use becks_match::*;

#[derive(Debug)]
pub struct MatLoaded {
    mat: Match,
    left: String,
    right: String,
}

#[derive(Debug)]
pub struct MatPanel {
    mat: Arc<mat::MatchList>,
    loaded: Vec<MatLoaded>,
    is_loaded: bool,
    /// The focus crew id, if any
    focus: Option<Id>,
}

#[derive(Debug, Clone)]
pub enum MatMessage {
    Reload,
    Load,
    Loaded(Acquire<Vec<MatLoaded>>),
    View(usize),
}

impl MatPanel {
    pub fn new(list: mat::MatchList, focus: Option<Id>) -> Self {
        Self {
            mat: Arc::new(list),
            loaded: Default::default(),
            is_loaded: false,
            focus,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }
}

impl Panel for MatPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::MatMessage(message) => match message {
                MatMessage::Reload => {
                    let mat = self.mat.clone();
                    Task::perform(
                        async move {
                            let result: Result<Vec<MatLoaded>> = match mat
                                .reload(login.as_ref())
                                .await
                            {
                                Ok(_) => {
                                    let mut result = Vec::new();
                                    for mat in mat.iter() {
                                        result.push(
                                            mat.write().await.load(login.as_ref()).await.cloned(),
                                        );
                                    }
                                    let mat = result.into_iter().collect::<Result<Vec<_>>>();
                                    match mat {
                                        Ok(mat) => {
                                            let mut result = Vec::new();
                                            for mat in mat.into_iter() {
                                                if let Ok(left) = crew::CrewInfo::new(mat.left)
                                                    .load(login.as_ref())
                                                    .await
                                                    .map(|crew| crew.name.clone())
                                                {
                                                    if let Ok(right) =
                                                        crew::CrewInfo::new(mat.right)
                                                            .load(login.as_ref())
                                                            .await
                                                            .map(|crew| crew.name.clone())
                                                    {
                                                        result.push(MatLoaded { mat, left, right })
                                                    }
                                                }
                                            }
                                            Ok(result)
                                        }
                                        Err(err) => Err(err),
                                    }
                                }
                                Err(err) => Err(err),
                            };
                            result
                        },
                        |result| match result {
                            Ok(loaded) => {
                                MainMessage::MatMessage(MatMessage::Loaded(Acquire::new(loaded)))
                            }
                            Err(err) => {
                                error!("When loading matches panel, {}", err);
                                MainMessage::None
                            }
                        },
                    )
                }
                MatMessage::Load => {
                    if self.is_loaded {
                        Task::none()
                    } else {
                        Task::done(MainMessage::MatMessage(MatMessage::Reload))
                    }
                }
                MatMessage::Loaded(loaded) => {
                    if let Some(loaded) = loaded.try_acquire() {
                        self.is_loaded = true;
                        self.loaded = loaded;
                    }
                    Task::none()
                }
                MatMessage::View(index) => {
                    if let Some(mat) = self.loaded.get(index) {
                        todo!()
                    } else {
                        Task::none()
                    }
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<MainMessage> {
        let poster_view: Element<MainMessage> = if self.is_loaded {
            if self.mat.is_empty() {
                widget::text(assets::TEXT.get("mat_empty")).into()
            } else {
                let mut column: Vec<Element<MainMessage>> = Vec::new();
                for (index, mat) in self.loaded.iter().enumerate() {
                    column.push(view_mat(mat, self.focus));
                    // column.push(
                    //     widget::button(widget::image("assets/jump.png"))
                    //         .height(25)
                    //         .on_press(MainMessage::MatMessage(MatMessage::View(index)))
                    //         .into(),
                    // );
                    column.push(widget::Rule::horizontal(2).into());
                }
                widget::scrollable(widget::Column::from_iter(column)).into()
            }
        } else {
            widget::text(assets::TEXT.get("mat_loading"))
                .style(widget::text::secondary)
                .into()
        };
        widget::column![
            widget::text(assets::TEXT.get("mat_title")).style(widget::text::primary),
            poster_view
        ]
        .into()
    }
}

fn view_mat(mat: &MatLoaded, focus: Option<Id>) -> Element<MainMessage> {
    let mut row: Vec<Element<MainMessage>> = Vec::new();
    row.push(widget::text(format!("{} vs. {}", mat.left, mat.right)).into());
    match mat.mat.quit {
        Quit::Normal => {
            let left_wins =
                mat.mat.rounds.iter().fold(
                    0i32,
                    |sum, round| if round.left_win { sum + 1 } else { sum },
                );
            let right_wins = mat.mat.total_rounds as i32 - left_wins;
            row.push(widget::text(format!("{} : {}", left_wins, right_wins)).into());
        }
        Quit::LeftQuit => {
            row.push(
                widget::text(format!(
                    "{}; 0 : {}",
                    assets::TEXT.get("mat_left_quit"),
                    mat.mat.total_rounds
                ))
                .into(),
            );
        }
        Quit::RightQuit => {
            row.push(
                widget::text(format!(
                    "{}; {} : 0",
                    assets::TEXT.get("mat_right_quit"),
                    mat.mat.total_rounds
                ))
                .into(),
            );
        }
    }
    if let Some(focus) = focus {
        let mut earn = 0;
        if mat.mat.left == focus {
            earn += mat.mat.left_earn;
        }
        if mat.mat.right == focus {
            earn += mat.mat.right_earn;
        }
        row.push(
            widget::text(format!("{}{}", if earn >= 0 { "+" } else { "" }, earn))
                .style(if earn >= 0 {
                    widget::text::success
                } else {
                    widget::text::danger
                })
                .into(),
        )
    }
    widget::Row::from_iter(row).spacing(15).padding(10).into()
}
