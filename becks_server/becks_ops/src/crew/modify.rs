use crate::prelude::*;

pub trait Column: Sized {
    type Target: Default + rusqlite::ToSql + rusqlite::types::FromSql;
    fn name() -> &'static str;
    fn convert(self) -> Self::Target;
    fn acquire(value: Self::Target) -> Self;
    /// Returns true if modification is successful
    fn modify(self, login: &Login, id: CrewId) -> bool {
        login
            .db
            .lock()
            .unwrap()
            .execute(
                indoc! {"
                    UPDATE crew
                    SET (:column) = (:value)
                    WHERE id = (:id)
                "},
                rusqlite::named_params! {
                    ":column": Self::name(),
                    ":value": self.convert(),
                    ":id":id.to_prim(),
                },
            )
            .inspect_err(|err| {
                error!("When modifying column {}, {}", Self::name(), err);
            })
            .is_ok()
    }
}

impl Column for String {
    type Target = String;
    fn name() -> &'static str {
        "name"
    }
    fn convert(self) -> Self::Target {
        self
    }
    fn acquire(value: Self::Target) -> Self {
        value
    }
}

impl Column for Social {
    type Target = u8;
    fn name() -> &'static str {
        "social"
    }
    fn convert(self) -> Self::Target {
        self.into()
    }
    fn acquire(value: Self::Target) -> Self {
        value.try_into().unwrap_or_else(|err| {
            error!("When converting database, {err}");
            Self::default()
        })
    }
}

impl Column for Gender {
    type Target = u8;
    fn name() -> &'static str {
        "gender"
    }
    fn convert(self) -> Self::Target {
        self.into()
    }
    fn acquire(value: Self::Target) -> Self {
        value.try_into().unwrap_or_else(|err| {
            error!("When converting database, {err}");
            Self::default()
        })
    }
}

impl Column for Clothes {
    type Target = u8;
    fn name() -> &'static str {
        "clothes"
    }
    fn convert(self) -> Self::Target {
        self.into()
    }
    fn acquire(value: Self::Target) -> Self {
        value.try_into().unwrap_or_else(|err| {
            error!("When converting database, {err}");
            Self::default()
        })
    }
}

impl Column for Hand {
    type Target = u8;
    fn name() -> &'static str {
        "hand"
    }
    fn convert(self) -> Self::Target {
        self.into()
    }
    fn acquire(value: Self::Target) -> Self {
        value.try_into().unwrap_or_else(|err| {
            error!("When converting database, {err}");
            Self::default()
        })
    }
}

impl Column for Hold {
    type Target = u8;
    fn name() -> &'static str {
        "hold"
    }
    fn convert(self) -> Self::Target {
        self.into()
    }
    fn acquire(value: Self::Target) -> Self {
        value.try_into().unwrap_or_else(|err| {
            error!("When converting database, {err}");
            Self::default()
        })
    }
}

impl Column for Paddle {
    type Target = String;
    fn name() -> &'static str {
        "paddle"
    }
    fn convert(self) -> Self::Target {
        check!(alnum self.brand);
        check!(alnum self.kind);
        format!("{}/{}", self.brand, self.kind)
    }
    fn acquire(value: Self::Target) -> Self {
        let values = value.splitn(2, "/").collect::<Vec<_>>();
        if values.len() != 2 {
            error!("Paddle information is erroneously stored: {}", value);
            Self::default()
        } else {
            Self {
                brand: values.first().unwrap().to_string(),
                kind: values.last().unwrap().to_string(),
            }
        }
    }
}

impl Column for RedRubber {
    type Target = String;
    fn name() -> &'static str {
        "red_rubber"
    }
    fn convert(self) -> Self::Target {
        check!(alnum self.0.brand);
        check!(alnum self.0.kind);
        format!("{}/{}", self.0.brand, self.0.kind)
    }
    fn acquire(value: Self::Target) -> Self {
        let values = value.splitn(2, "/").collect::<Vec<_>>();
        if values.len() != 2 {
            error!("Red rubber information is erroneously stored: {}", value);
            Self::default()
        } else {
            Self(Rubber {
                brand: values.first().unwrap().to_string(),
                kind: values.last().unwrap().to_string(),
            })
        }
    }
}

impl Column for BlackRubber {
    type Target = String;
    fn name() -> &'static str {
        "black_rubber"
    }
    fn convert(self) -> Self::Target {
        check!(alnum self.0.brand);
        check!(alnum self.0.kind);
        format!("{}/{}", self.0.brand, self.0.kind)
    }
    fn acquire(value: Self::Target) -> Self {
        let values = value.splitn(2, "/").collect::<Vec<_>>();
        if values.len() != 2 {
            error!("Black rubber information is erroneously stored: {}", value);
            Self::default()
        } else {
            Self(Rubber {
                brand: values.first().unwrap().to_string(),
                kind: values.last().unwrap().to_string(),
            })
        }
    }
}
