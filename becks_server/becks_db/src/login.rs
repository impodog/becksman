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
                    id INTEGER PRIMARY KEY,
                    name VARCHAR(20),
                    social BIT,
                    score INT,
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
                indoc! {"
                    CREATE INDEX idx_name ON crew (name);
                    CREATE INDEX idx_score ON crew (score)
                "},
                [],
            )
            .inspect_err(|err| {
                error!("When creating crew indices, {}", err);
            })
            .ok();
        }

        if !table_exists(&db, "round") {
            db.execute(
                indoc! {"
                    CREATE TABLE IF NOT EXISTS round (
                        id INTEGER PRIMARY KEY,
                        left_win BIT
                    )
                "},
                [],
            )
            .inspect_err(|err| {
                error!("When initializing round database, {}", err);
            })
            .ok();
        }

        if !table_exists(&db, "match") {
            db.execute(
                indoc! {"
                CREATE TABLE IF NOT EXISTS match (
                    id INTEGER PRIMARY KEY,
                    left INTEGER,
                    right INTEGER,
                    round_worth INT UNSIGNED,
                    rounds TEXT,
                    notes TEXT
                )
            "},
                [],
            )
            .inspect_err(|err| {
                error!("When initializing match database, {}", err);
            })
            .ok();
            db.execute(
                indoc! {"
                    CREATE INDEX idx_left ON match (left);
                    CREATE INDEX idx_right ON match (right);
                    CREATE INDEX idx_notes ON match (notes)
                "},
                [],
            )
            .inspect_err(|err| {
                error!("When creating match indices, {}", err);
            })
            .ok();
        }

        if !table_exists(&db, "poster") {
            db.execute(
                indoc! {"
                    CREATE TABLE IF NOT EXISTS poster (
                        id INTEGER PRIMARY KEY,
                        value TEXT,
                        compiled TEXT,
                        modified BIT,
                        timestamp INTEGER
                    )
                "},
                [],
            )
            .inspect_err(|err| {
                error!("When initializing poster database, {}", err);
            })
            .ok();
            db.execute(
                indoc! {"
                    CREATE INDEX idx_value ON poster (value);
                    CREATE INDEX idx_timestamp ON poster (timestamp)
                "},
                [],
            )
            .inspect_err(|err| {
                error!("When creating poster indices, {}", err);
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
