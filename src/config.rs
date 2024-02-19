use toml::ser::Error;

pub const CONFIG_FILE_NAME: &str = "dot_config.toml";

pub type Config = toml::Table;

pub fn load(buffer: &str) -> Result<Config, Error> {
    toml::Table::try_from(buffer)
}

pub fn save(config: &Config) -> Result<String, Error> {
    toml::to_string(config)
}

pub fn insert(config: &Config, file: String, link: String) -> Config {
    let mut updated_config = config.clone();
    updated_config.insert(file, toml::Value::String(link));
    updated_config
}
