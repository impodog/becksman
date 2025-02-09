use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, RwLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Interact {
    pub recent: std::time::Duration,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            recent: std::time::Duration::new(604800, 0),
        }
    }
}

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
pub struct Assets {
    pub icon: String,
    pub text: String,
    pub fonts: Vec<String>,
    pub primary_font: String,
}
impl Default for Assets {
    fn default() -> Self {
        Assets {
            icon: "assets/icon.bmp".to_owned(),
            text: "assets/zh_cn.ini".to_owned(),
            fonts: vec![
                "assets/JetBrains.ttf".to_owned(),
                "assets/Hack.ttf".to_owned(),
                "assets/NotoSans.ttc".to_owned(),
            ],
            primary_font: "Noto Sans CJK SC".to_owned(),
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
    pub interact: Interact,
    pub client: Client,
    pub assets: Assets,
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
macro_rules! server_url {
    ($route: expr) => {
        format!("http://{}/{}", $crate::config::CONFIG.client.addr, $route)
    };
}
