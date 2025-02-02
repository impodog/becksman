use crate::prelude::*;
use crate::CONFIG;
use std::sync::RwLock;

pub struct Db {
    user: Connection,
    login: RwLock<crate::LoginMap>,
}

impl Db {
    pub fn connect() -> Self {
        info!("Connecting to main database");
        let user = Connection::open(&CONFIG.db.becksman).unwrap_or_else(|err| {
            error!(
                "When opening main database {:?}, {}",
                CONFIG.db.becksman, err
            );
            warn!(
                "Opening main database in memory, you may lose all data after closing this program"
            );
            Connection::open_in_memory().expect("rusqlite should connect to the database")
        });
        Self {
            user,
            login: Default::default(),
        }
    }

    /// Attempts to create a new user, return true if a user is created
    pub fn create(&self, name: &str, pass: &str) -> bool {
        check!(alnum name);
        check!(alnum pass);
        if self
            .user
            .query_row("SELECT * FROM user WHERE name = ?1", [name], |_| Ok(()))
            .is_ok()
        {
            // Replicate users
            false
        } else {
            self.user
                .execute(
                    "INSERT INTO user (name, pass) values (?1, ?2)",
                    [name, pass],
                )
                .inspect_err(|err| {
                    error!("When creating user, {}", err);
                })
                .is_ok()
        }
    }

    /// Attempts to log in to the program, returns [`Some`] if a token is given
    pub fn log_in(&self, name: &str, pass: &str) -> Option<Token> {
        check!(alnum name);
        check!(alnum pass);
        info!("Attempt to log in with name {}, password {}", name, pass);
        let target = self
            .user
            .query_row("SELECT pass FROM user WHERE name = ?1", [name], |row| {
                row.get::<_, String>("pass")
            });
        if let Ok(target) = target {
            if target != pass {
                error!("Password {} is wrong for user {}", pass, name);
                return None;
            }
            let mut login = self.login.write().unwrap();
            let value = crate::Login::new(name.to_owned());
            login.insert(value)
        } else {
            error!("User {} is not found", name);
            None
        }
    }

    /// Attempts to log out of the program, returns true on success
    pub fn log_out(&self, token: Token) -> bool {
        let mut login = self.login.write().unwrap();
        if let Some(login) = login.remove(token) {
            info!("User {} with token {:?} logged out", login.name, token);
            true
        } else {
            error!("When logging out, token {:?} cannot be found", token);
            false
        }
    }
}
