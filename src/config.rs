use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use toml::{de, ser};

pub const CONFIG_FILE_NAME: &str = "dot_config.toml";

pub type Config = HashMap<PathBuf, PathBuf>;

pub fn load(buffer: &str) -> Result<Config, de::Error> {
    toml::from_str(buffer)
}

pub fn save(config: &Config) -> Result<String, ser::Error> {
    toml::to_string(config)
}

pub fn insert(config: &Config, file: PathBuf, link: PathBuf) -> Config {
    let mut updated_config = config.clone();
    updated_config.insert(file, link);
    updated_config
}

pub fn remove(config: &Config, file: &PathBuf) -> Config {
    let mut updated_config = config.clone();
    updated_config.remove(file);
    updated_config
}

pub fn get(config: &Config, file: &Path) -> Option<PathBuf> {
    config.get(file).clone().map(PathBuf::to_owned)
}

pub fn has(config: &Config, file: &Path) -> bool {
    config.get(file).is_some()
}
