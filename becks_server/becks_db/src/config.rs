use crate::prelude::*;
use std::path::PathBuf;
use std::sync::LazyLock;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub file: PathBuf,
    pub addr: std::net::SocketAddr,
}
impl Default for Database {
    fn default() -> Self {
        Self {
            file: PathBuf::from("becksman.db"),
            addr: "127.0.0.1:114".parse().expect("should be valid address"),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub db: Database,
}

impl Config {
    pub const CONFIG_PATH: &'static str = "becksman.toml";

    pub fn read_local() -> Config {
        info!("Reading configuration from {:?}", Self::CONFIG_PATH);
        let config = std::fs::read_to_string(Self::CONFIG_PATH).unwrap_or_else(|err| {
            error!(
                "When reading configuration from {:?}, {}",
                Self::CONFIG_PATH,
                err
            );
            Default::default()
        });
        toml::from_str::<Config>(&config).unwrap_or_else(|err| {
            error!("When parsing string:\n{}\n{}", config, err);
            Default::default()
        })
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::read_local);
