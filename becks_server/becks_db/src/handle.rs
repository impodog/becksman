use crate::prelude::*;
use crate::CONFIG;

pub struct Db {
    user: rusqlite::Connection,
}

impl Db {
    pub fn connect() -> Self {
        info!("Connecting to database");
        let user = rusqlite::Connection::open(&CONFIG.db.file).unwrap_or_else(|err| {
            error!("When opening user database {:?}, {}", CONFIG.db.file, err);
            warn!(
                "Opening user database in memory, you may lose all data after closing this program"
            );
            rusqlite::Connection::open_in_memory().expect("rusqlite should connect to the database")
        });
        Self { user }
    }
}
