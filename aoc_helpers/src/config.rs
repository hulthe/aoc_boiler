use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub session: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let mut config_data = String::new();
    let mut file = File::open("config.toml")?;
    file.read_to_string(&mut config_data)?;
    Ok(toml::from_str(&config_data)?)
}
