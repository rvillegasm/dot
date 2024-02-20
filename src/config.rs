use std::collections::HashMap;

use toml::{de, ser};

pub const CONFIG_FILE_NAME: &str = ".dot_config.toml";

pub type Config = HashMap<String, String>;

pub fn load(buffer: &str) -> Result<Config, de::Error> {
    toml::from_str(buffer)
}

pub fn save(config: &Config) -> Result<String, ser::Error> {
    toml::to_string(config)
}

pub fn insert(config: &Config, file: String, link: String) -> Config {
    let mut updated_config = config.clone();
    updated_config.insert(file, link);
    updated_config
}
