pub(crate) use becks_crew::*;
pub(crate) use becks_db::Login;
pub(crate) use indoc::{formatdoc, indoc};
pub(crate) use log::{debug, error, info, trace, warn};
pub(crate) use rusqlite::ToSql;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use thiserror::Error;

pub(crate) fn box_sql(value: impl ToSql + 'static) -> Box<dyn ToSql> {
    Box::new(value)
}
