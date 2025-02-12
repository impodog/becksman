use crate::prelude::*;

#[derive(Debug, Default)]
pub struct PosterCreatePanel {
    value: String,
    images: Vec<String>,
    error: bool,
}

#[derive(Debug, Clone)]
pub enum PosterCreateMessage {
    UpdateValue(String),
    PickImage,
    AddImages(Vec<String>),
    RemoveImage(usize),
    Create,
    Created(Acquire<poster::PosterInfo>),
    CreateError,
}

impl PosterCreatePanel {
    fn view_images(&self) -> Element<MainMessage> {
        let mut column: Vec<Element<MainMessage>> = Vec::new();
        for (index, image) in self.images.iter().enumerate() {
            column.push(
                widget::row![
                    widget::image(image),
                    widget::horizontal_space(),
                    widget::button(widget::image("assets/remove.png"))
                        .on_press(MainMessage::PosterCreateMessage(
                            PosterCreateMessage::RemoveImage(index)
                        ))
                        .height(25)
                ]
                .height(100)
                .into(),
            )
        }
        if column.is_empty() {
            column.push(widget::horizontal_space().into());
        }
        column.push(
            widget::button(widget::image("assets/add.png"))
                .on_press(MainMessage::PosterCreateMessage(
                    PosterCreateMessage::PickImage,
                ))
                .height(25)
                .into(),
        );
        widget::Column::from_iter(column)
            .padding(10)
            .spacing(10)
            .into()
    }
}

impl Panel for PosterCreatePanel {
    fn update_with_login(&mut self, login: Arc<Login>, message: MainMessage) -> Task<MainMessage> {
        self.error = false;
        match message {
            MainMessage::PosterCreateMessage(message) => match message {
                PosterCreateMessage::UpdateValue(value) => {
                    self.value = value;
                    Task::none()
                }
                PosterCreateMessage::PickImage => Task::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .add_filter("", &["png", "jpg", "jpeg", "bmp"])
                            .set_title(assets::TEXT.get("poster_create_image_title"))
                            .pick_files()
                            .await
                    },
                    |handles| {
                        if let Some(handles) = handles {
                            let images = handles
                                .into_iter()
                                .filter_map(|handle| {
                                    handle
                                        .path()
                                        .canonicalize()
                                        .ok()
                                        .map(|path| path.to_string_lossy().into_owned())
                                })
                                .collect();
                            MainMessage::PosterCreateMessage(PosterCreateMessage::AddImages(images))
                        } else {
                            MainMessage::None
                        }
                    },
                ),
                PosterCreateMessage::AddImages(images) => {
                    self.images.extend(images);
                    Task::none()
                }
                PosterCreateMessage::RemoveImage(index) => {
                    if index < self.images.len() {
                        self.images.remove(index);
                    }
                    Task::none()
                }
                PosterCreateMessage::Create => {
                    let value = self.value.clone();
                    let images = self.images.clone();
                    Task::perform(
                        async move { poster::PosterInfo::create(login.as_ref(), value, images).await },
                        |result| match result {
                            Ok(poster) => MainMessage::PosterCreateMessage(
                                PosterCreateMessage::Created(Acquire::new(poster)),
                            ),
                            Err(err) => {
                                warn!("When creating poster, {}", err);
                                MainMessage::PosterCreateMessage(PosterCreateMessage::CreateError)
                            }
                        },
                    )
                }
                PosterCreateMessage::Created(poster) => {
                    if let Some(poster) = poster.try_acquire() {
                        Task::done(MainMessage::RewindThen(Acquire::new(PanelHandle::new(
                            poster_view::PosterViewPanel::new(poster),
                        ))))
                    } else {
                        Task::none()
                    }
                }
                PosterCreateMessage::CreateError => {
                    self.error = true;
                    Task::none()
                }
            },
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<MainMessage> {
        let sub_column = widget::column![
            widget::button(assets::TEXT.get("poster_create_create")).on_press(
                MainMessage::PosterCreateMessage(PosterCreateMessage::Create)
            ),
            widget::text_input(assets::TEXT.get("poster_create_input_hint"), &self.value).on_input(
                |value| MainMessage::PosterCreateMessage(PosterCreateMessage::UpdateValue(value))
            ),
            widget::container(widget::scrollable(self.view_images()).height(200))
                .style(widget::container::rounded_box),
        ]
        .push_maybe(if self.error {
            Some(widget::text(assets::TEXT.get("poster_create_error")).style(widget::text::danger))
        } else {
            None
        });
        widget::column![
            widget::text(assets::TEXT.get("poster_create_title")),
            sub_column
        ]
        .into()
    }
}
