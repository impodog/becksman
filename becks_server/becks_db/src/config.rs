use crate::prelude::*;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Clone)]
pub struct Database {
    pub becksman: PathBuf,
    pub user_base: PathBuf,
}
impl Default for Database {
    fn default() -> Self {
        Self {
            becksman: PathBuf::from("_becksman.db"),
            user_base: PathBuf::from("./"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub addr: std::net::SocketAddr,
}
impl Default for Server {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:1145".parse().expect("should be valid address"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub timeout: std::time::Duration,
    pub elo_scaler: f32,
}
impl Default for User {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::new(60, 0),
            elo_scaler: 3.0,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Config {
    pub db: Database,
    pub server: Server,
    pub user: User,
}

impl Config {
    pub const CONFIG_PATH: &'static str = "becksman-server.toml";

    pub fn invoke_lazy(&self) {}

    pub fn read_local() -> Config {
        trace!("Reading configuration from {:?}", Self::CONFIG_PATH);
        let config = std::fs::read_to_string(Self::CONFIG_PATH).unwrap_or_else(|err| {
            warn!(
                "When reading configuration from {:?}, {}; Using default",
                Self::CONFIG_PATH,
                err
            );
            *SAVE_CONFIG.write().unwrap() = Some(Config::default());
            Default::default()
        });
        toml::from_str::<Config>(&config).unwrap_or_else(|err| {
            warn!("When parsing string:\n{}\n{}; Using default", config, err);
            Default::default()
        })
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::read_local);
pub static SAVE_CONFIG: RwLock<Option<Config>> = RwLock::new(None);

pub fn save_config() {
    CONFIG.invoke_lazy();
    let lock = SAVE_CONFIG.read().unwrap();
    if let Some(config) = &*lock {
        let value = toml::to_string_pretty(config).expect("serialization should succeed");
        std::fs::write(Config::CONFIG_PATH, value).unwrap_or_else(|err| {
            error!(
                "When writing configuration to {:?}, {}",
                Config::CONFIG_PATH,
                err
            );
        });
    }
}
