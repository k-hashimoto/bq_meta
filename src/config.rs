use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

use crate::models::Config;

pub fn get_data_path() -> Result<PathBuf> {
    if let Ok(path) = env::var("BQ_META_PATH") {
        Ok(PathBuf::from(path))
    } else {
        let home = dirs::home_dir()
            .context("Could not determine home directory")?;
        Ok(home.join(".bq-meta"))
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let data_path = get_data_path()?;
    Ok(data_path.join("config.yaml"))
}

pub fn get_data_dir() -> Result<PathBuf> {
    let data_path = get_data_path()?;
    Ok(data_path.join("data"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
    
    let config: Config = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;
    
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
    }

    let content = serde_yaml::to_string(config)
        .context("Failed to serialize config")?;
    
    std::fs::write(&config_path, content)
        .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;
    
    Ok(())
}

pub fn init_data_directory() -> Result<()> {
    let data_path = get_data_path()?;
    let data_dir = get_data_dir()?;
    
    std::fs::create_dir_all(&data_dir)
        .with_context(|| format!("Failed to create data directory: {}", data_dir.display()))?;
    
    // Create default config if it doesn't exist
    let config_path = get_config_path()?;
    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
    }

    println!("Initialized bq-meta data directory at: {}", data_path.display());
    Ok(())
}