use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub addr: std::net::SocketAddr,
}
impl Default for Client {
    fn default() -> Self {
        Self {
            addr: std::net::SocketAddr::new(
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
                1145,
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub update_relay: std::time::Duration,
}
impl Default for Request {
    fn default() -> Self {
        Self {
            update_relay: std::time::Duration::new(40, 0),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub client: Client,
    pub request: Request,
}

impl Config {
    pub const CONFIG_PATH: &'static str = "becksman-client.toml";

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
        let value = toml::to_string(config).expect("serialization should succeed");
        std::fs::write(Config::CONFIG_PATH, value).unwrap_or_else(|err| {
            error!(
                "When writing configuration to {:?}, {}",
                Config::CONFIG_PATH,
                err
            );
        });
    }
}

#[macro_export]
macro_rules! url {
    ($route: expr) => {
        format!("{}/{}", $crate::config::CONFIG.client.addr, $route)
    };
}
