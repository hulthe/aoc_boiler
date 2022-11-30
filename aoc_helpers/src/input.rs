use crate::config::{load_config, Config};
use anyhow::format_err;
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;

fn download_input(year: u32, day: u8, config: &Config) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    Ok(client
        .get(&format!("{}/{}/day/{}/input", config.url, year, day))
        .header("cookie", format!("session={}", config.session))
        .send()?
        .error_for_status()?
        .text()?)
}

fn write_input_to_cache(cache_path: &Path, input: &str) -> anyhow::Result<()> {
    let mut file = File::create(cache_path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}

fn get_input_from_cache(cache_path: &Path) -> anyhow::Result<String> {
    let mut cache_data = String::new();
    let mut file = File::open(cache_path)?;
    file.read_to_string(&mut cache_data)?;
    Ok(cache_data)
}

pub fn get_input(year: u32, day: u8) -> anyhow::Result<String> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))?;
    let config = load_config().map_err(|e| format_err!("Failed to load config.toml: {e}"))?;

    let mut hasher = DefaultHasher::new();
    config.session.hash(&mut hasher);
    let session_hash = hex::encode(&hasher.finish().to_be_bytes());

    let cache_folder = format!("{}/{}", year, day);
    let mut cache_path = xdg_dirs.create_cache_directory(cache_folder)?;
    cache_path.push(session_hash);

    if let Ok(input) = get_input_from_cache(&cache_path) {
        Ok(input)
    } else {
        let input = download_input(year, day, &config)?;
        write_input_to_cache(&cache_path, &input)?;
        Ok(input)
    }
}
