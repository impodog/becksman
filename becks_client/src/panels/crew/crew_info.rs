use crate::prelude::*;
use becks_crew::*;
use crew_repr::Brand;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct CrewInfoPanel {
    crew: Arc<Mutex<crew::CrewInfo>>,
    crew_data: Option<CrewData>,
    error: bool,
    delete_confirm: bool,
}

#[derive(Debug, Clone)]
pub enum CrewInfoMessage {
    Load,
    Loaded(Acquire<CrewData>),
    LoadError,
    Update(CrewLocation),
    DeleteConfirm,
    Delete,
}

impl CrewInfoPanel {
    pub fn new(id: Id) -> Self {
        Self {
            crew: Arc::new(Mutex::new(crew::CrewInfo::new(id))),
            crew_data: None,
            error: false,
            delete_confirm: false,
        }
    }
}

macro_rules! view_kv {
    ($data: ident, $view_key: literal, $hint_key: literal, $construct: expr, $acquire: expr, $loc: ident, $data_field: ident) => {{
        let construct = $construct;
        let acquire = $acquire;
        view_data(
            $view_key,
            widget::row![
                widget::pick_list(
                    Brand::all_repred(),
                    $data
                        .$data_field
                        .as_ref()
                        .map(|paddle| Brand::from_server(&acquire(paddle).brand).repr()),
                    move |brand| {
                        let brand = Brand::unrepr(brand);
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::$loc(
                            construct(
                                brand.to_server(),
                                $data
                                    .$data_field
                                    .as_ref()
                                    .map_or_else(Default::default, |paddle| {
                                        acquire(paddle).kind.clone()
                                    }),
                            ),
                        )))
                    }
                )
                .width(100),
                widget::text_input(
                    assets::TEXT.get($hint_key),
                    $data
                        .$data_field
                        .as_ref()
                        .map_or("", |paddle| acquire(paddle).kind.as_ref())
                )
                .on_input(move |value| {
                    MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::$loc(
                        construct(
                            $data
                                .$data_field
                                .as_ref()
                                .map_or_else(Default::default, |paddle| {
                                    acquire(paddle).brand.clone()
                                }),
                            value,
                        ),
                    )))
                })
            ],
        )
    }};
}

impl Panel for CrewInfoPanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        match message {
            MainMessage::CrewInfoMessage(message) => match message {
                CrewInfoMessage::Load => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move { crew.lock().await.load(login.as_ref()).await.cloned() },
                        |crew| match crew {
                            Ok(crew_data) => MainMessage::CrewInfoMessage(CrewInfoMessage::Loaded(
                                Acquire::new(crew_data),
                            )),
                            Err(err) => {
                                warn!("When loading crew message, {}", err);
                                MainMessage::CrewInfoMessage(CrewInfoMessage::LoadError)
                            }
                        },
                    )
                }
                CrewInfoMessage::Loaded(crew_data) => {
                    if let Some(crew_data) = crew_data.try_acquire() {
                        self.crew_data = Some(crew_data);
                        self.error = false;
                    }
                    Task::none()
                }
                CrewInfoMessage::Update(loc) => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move { crew.lock().await.modify(login.as_ref(), loc).await },
                        |result| {
                            if let Err(err) = result {
                                error!("When modifying crew {}", err);
                            }
                            MainMessage::CrewInfoMessage(CrewInfoMessage::Load)
                        },
                    )
                }
                CrewInfoMessage::LoadError => {
                    self.error = true;
                    Task::none()
                }
                CrewInfoMessage::DeleteConfirm => {
                    self.delete_confirm = true;
                    Task::none()
                }
                CrewInfoMessage::Delete => {
                    let crew = self.crew.clone();
                    Task::perform(
                        async move { crew.lock().await.delete(login.as_ref()).await },
                        |result| match result {
                            Ok(_) => MainMessage::Rewind,
                            Err(err) => {
                                warn!("When deleting crew, {}", err);
                                MainMessage::CrewInfoMessage(CrewInfoMessage::LoadError)
                            }
                        },
                    )
                }
            },
            _ => {
                todo!()
            }
        }
    }

    fn view(&self) -> Element<MainMessage> {
        let mut column: Vec<Element<MainMessage>> = Vec::new();
        column.push(widget::text(assets::TEXT.get("crew_info_title")).into());
        if let Some(data) = self.crew_data.as_ref() {
            column.push(view_data(
                "crew_info_name",
                widget::text_input(assets::TEXT.get("crew_info_name"), &data.name).on_input(
                    |name| {
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Name(
                            name,
                        )))
                    },
                ),
            ));
            column.push(view_data(
                "crew_info_social",
                widget::pick_list(Social::all_repred(), Some(data.social.repr()), |social| {
                    let social = Social::unrepr(social);
                    MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Social(
                        *social,
                    )))
                }),
            ));
            column.push(view_data(
                "crew_info_gender",
                widget::pick_list(
                    Gender::all_repred(),
                    data.gender.as_ref().map(Repr::repr),
                    |gender| {
                        let gender = Gender::unrepr(gender);
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Gender(
                            *gender,
                        )))
                    },
                ),
            ));
            column.push(view_data(
                "crew_info_score",
                widget::text(data.score.0.to_string()).style(widget::text::success),
            ));
            column.push(view_data(
                "crew_info_hold",
                widget::pick_list(
                    Hold::all_repred(),
                    data.hold.as_ref().map(Repr::repr),
                    |hold| {
                        let hold = Hold::unrepr(hold);
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Hold(
                            *hold,
                        )))
                    },
                ),
            ));
            column.push(view_data(
                "crew_info_hand",
                widget::pick_list(
                    Hand::all_repred(),
                    data.hand.as_ref().map(Repr::repr),
                    |hand| {
                        let hand = Hand::unrepr(hand);
                        MainMessage::CrewInfoMessage(CrewInfoMessage::Update(CrewLocation::Hand(
                            *hand,
                        )))
                    },
                ),
            ));
            fn construct_paddle(brand: String, kind: String) -> Paddle {
                Paddle { brand, kind }
            }
            fn acquire_paddle(paddle: &Paddle) -> &Paddle {
                paddle
            }
            column.push(view_kv!(
                data,
                "crew_info_paddle",
                "crew_info_paddle_kind_hint",
                construct_paddle,
                acquire_paddle,
                Paddle,
                paddle
            ));
            fn construct_red(brand: String, kind: String) -> RedRubber {
                RedRubber(Rubber { brand, kind })
            }
            fn acquire_red(red: &RedRubber) -> &Rubber {
                &red.0
            }
            column.push(view_kv!(
                data,
                "crew_info_red",
                "crew_info_red_kind_hint",
                construct_red,
                acquire_red,
                Red,
                red
            ));
            fn construct_black(brand: String, kind: String) -> BlackRubber {
                BlackRubber(Rubber { brand, kind })
            }
            fn acquire_black(black: &BlackRubber) -> &Rubber {
                &black.0
            }
            column.push(view_kv!(
                data,
                "crew_info_black",
                "crew_info_black_kind_hint",
                construct_black,
                acquire_black,
                Black,
                black
            ));

            column.push(
                widget::button(if self.delete_confirm {
                    assets::TEXT.get("crew_info_delete_confirm")
                } else {
                    assets::TEXT.get("crew_info_delete")
                })
                .style(widget::button::danger)
                .on_press(if self.delete_confirm {
                    MainMessage::CrewInfoMessage(CrewInfoMessage::Delete)
                } else {
                    MainMessage::CrewInfoMessage(CrewInfoMessage::DeleteConfirm)
                })
                .into(),
            )
        } else {
            column.push(widget::text(assets::TEXT.get("crew_loading")).into());
        }
        if self.error {
            column.push(widget::horizontal_rule(2).into());
            column.push(
                widget::text("crew_info_error")
                    .style(widget::text::danger)
                    .into(),
            );
        }
        widget::scrollable(widget::Column::from_iter(column).spacing(10).padding(20))
            .direction(widget::scrollable::Direction::Vertical(
                widget::scrollable::Scrollbar::new(),
            ))
            .height(300)
            .into()
    }

    fn on_start_up(&mut self) -> Task<MainMessage> {
        Task::done(MainMessage::CrewInfoMessage(CrewInfoMessage::Load))
    }

    fn is_done_able(&self) -> bool {
        true
    }
}

fn view_data_key(key: &str) -> Element<'static, MainMessage> {
    widget::text(assets::TEXT.get(key))
        .style(widget::text::primary)
        .into()
}

fn view_data<'a>(key: &str, data: impl Into<Element<'a, MainMessage>>) -> Element<'a, MainMessage> {
    widget::row![view_data_key(key), widget::horizontal_space(), data.into()].into()
}
