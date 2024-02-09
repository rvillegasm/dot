use toml::ser::Error;

type Config = toml::Table;

pub fn load(buffer: &str) -> Result<Config, Error> {
    toml::Table::try_from(buffer)
}

fn insert(config: &Config, file: String, link: String) -> Config {
    let mut updated_config = config.clone();
    updated_config.insert(file, toml::Value::String(link));
    updated_config
}

pub fn write(config: &Config) -> Result<String, Error> {
    toml::to_string(config)
}
