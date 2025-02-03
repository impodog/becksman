use crate::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Mutex;

pub struct Login {
    pub name: String,
    pub db: Mutex<Connection>,
}

impl Login {
    /// Connects to the corresponding database
    pub fn new(name: String) -> Self {
        let path = CONFIG.db.user_base.join(format!("{}.db", name));
        info!("Connecting to user database {:?}", path);
        let db = Connection::open(&path).unwrap_or_else(|err| {
            error!("When opening user database {:?}, {}", path, err);
            warn!(
                "Opening user database in memory, you may lose all data after closing this program"
            );
            Connection::open_in_memory().expect("rusqlite should connect to the database")
        });
        Self {
            name,
            db: Mutex::new(db),
        }
    }
}

pub struct LoginMap {
    map: HashMap<Token, Login>,
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
    type Target = HashMap<Token, Login>;
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
            self.map.insert(token, login);
            Some(token)
        } else {
            None
        }
    }

    /// Removes the login entry of the user, or return None
    pub(crate) fn remove(&mut self, token: Token) -> Option<Login> {
        let login = self.map.remove(&token);
        if let Some(ref login) = login {
            self.logged.remove(&login.name);
        }
        login
    }
}
