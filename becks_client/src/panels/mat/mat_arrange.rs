use crate::arrange::*;
use crate::prelude::*;
use becks_match::*;
use mat_panel::MatMessage;

#[derive(Debug)]
pub struct MatArrangePanel {
    select: Option<crew_query::CrewQueryPanel>,
    selected: Option<Vec<Id>>,
    arranger: Option<Arranger>,
    group_size: usize,
    error: bool,
    local_error: bool,
}

#[derive(Debug, Clone)]
pub enum MatArrangeMessage {
    StartArrange,
    ArrangeAcquired(Acquire<Arranger>),
    StartSelection,
    EndSelection,
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
                        let group_size = self.group_size;
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
                        Task::done(MainMessage::MatArrangeMessage(
                            MatArrangeMessage::LocalError,
                        ))
                    }
                }
                MatArrangeMessage::ArrangeAcquired(arranger) => {
                    if let Some(mut arranger) = arranger.try_acquire() {
                        arranger.arrange();
                        self.arranger = Some(arranger);
                    }
                    Task::none()
                }
                MatArrangeMessage::StartSelection => {
                    self.select = Some(
                        crew_query::CrewQueryPanel::default()
                            .select_only()
                            .allow_select_all(),
                    );
                    Task::none()
                }
                MatArrangeMessage::EndSelection => {
                    if let Some(selection) = self.select.as_ref() {
                        self.selected = Some(selection.selection().iter().copied().collect());
                        Task::done(MainMessage::MatArrangeMessage(
                            MatArrangeMessage::StartArrange,
                        ))
                    } else {
                        Task::none()
                    }
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
                if let Some(selection) = self.select.as_mut() {
                    selection.update_with_login(login, message)
                } else {
                    Task::none()
                }
            }
        }
    }
    fn view(&self) -> Element<MainMessage> {
        todo!()
    }
}
