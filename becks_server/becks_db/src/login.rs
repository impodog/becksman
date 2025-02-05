use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Login {
    pub name: String,
    pub db: Mutex<Connection>,
}

/// Returns whether the table exists in a database file, returning true on sqlite errors
fn table_exists(conn: &Connection, table_name: &str) -> bool {
    // Query the sqlite_master table to check if the table exists
    match conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?") {
        Ok(mut stmt) => stmt.exists([table_name]).unwrap_or_else(|err| {
            error!("When checking table {} existence, {}", table_name, err);
            true
        }),
        Err(err) => {
            error!("When checking table {} existence, {}", table_name, err);
            true
        }
    }
}

impl Login {
    /// Connects to the corresponding database
    pub fn new(name: String) -> Self {
        let path = CONFIG.db.user_base.join(format!("{}.db", name));
        trace!("Connecting to user database {:?}", path);
        let db = Connection::open(&path).unwrap_or_else(|err| {
            error!(
                "When opening user database {:?}, {}; Opening database in memory",
                path, err
            );
            Connection::open_in_memory().expect("rusqlite should connect to the database")
        });

        if !table_exists(&db, "crew") {
            db.execute(
                indoc! {
                    "CREATE TABLE IF NOT EXISTS crew (
                    id BIGINT PRIMARY KEY,
                    name VARCHAR(20),
                    social BIT,
                    gender BIT,
                    clothes TINYINT,
                    hand BIT,
                    hold BIT,
                    paddle VARCHAR(40),
                    red_rubber VARCHAR(40),
                    black_rubber VARCHAR(40)
                )"
                },
                [],
            )
            .inspect_err(|err| {
                error!("When initializing crew database, {}", err);
            })
            .ok();
            db.execute(
                indoc! {
                    "CREATE INDEX idx_name ON crew (name)"
                },
                [],
            )
            .inspect_err(|err| {
                error!("When creating indices, {}", err);
            })
            .ok();
        }

        Self {
            name,
            db: Mutex::new(db),
        }
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock().unwrap()
    }
}

pub struct LoginMap {
    map: HashMap<Token, Arc<Login>>,
    logged: HashSet<String>,
}

impl Default for LoginMap {
    fn default() -> Self {
        trace!("LoginMap CREATED");
        Self {
            map: Default::default(),
            logged: Default::default(),
        }
    }
}

impl Deref for LoginMap {
    type Target = HashMap<Token, Arc<Login>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl LoginMap {
    /// Creates an empty login map
    pub fn new() -> Self {
        Self::default()
    }

    fn gen_token(&self) -> Token {
        loop {
            let token = Token::new(rand::random());
            if !self.map.contains_key(&token) {
                return token;
            }
        }
    }

    /// Inserts a new login into the map, returning its token on success;
    /// If the user has already logged in, None is returned
    pub(crate) fn insert(&mut self, login: Login) -> Option<Token> {
        if self.logged.insert(login.name.clone()) {
            let token = self.gen_token();
            self.map.insert(token, Arc::new(login));
            Some(token)
        } else {
            None
        }
    }

    /// Removes the login entry of the user, or return None
    pub(crate) fn remove(&mut self, token: Token) -> Option<Arc<Login>> {
        let login = self.map.remove(&token);
        if let Some(ref login) = login {
            self.logged.remove(&login.name);
        }
        login
    }
}
