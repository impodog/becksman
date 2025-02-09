use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Serialize, Deserialize)]
pub struct Text {
    map: HashMap<String, String>,
}

impl Text {
    pub fn from_conf(conf: &str) -> Self {
        let items = conf
            .split('\n')
            .filter_map(|value| {
                let value = value.trim();
                if value.is_empty() {
                    None
                } else if let Some(pos) = value.find('=') {
                    let (left, right) = value.split_at(pos);
                    Some((left.trim().to_owned(), right[1..].trim().to_owned()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Self {
            map: HashMap::from_iter(items),
        }
    }

    pub fn read(path: &str) -> Self {
        let conf = std::fs::read_to_string(path).unwrap_or_else(|err| {
            error!(
                "When reading text from {}, {}; All text would be unavailable!",
                path, err
            );
            Default::default()
        });
        Self::from_conf(&conf)
    }

    pub fn get(&self, key: &str) -> &str {
        self.map
            .get(key)
            .map(|s| s.as_ref())
            .unwrap_or_else(|| "<MISSING-TEXT>")
    }
}

pub static TEXT: LazyLock<Text> =
    LazyLock::new(|| Text::read(&becks_network::config::CONFIG.assets.text));
