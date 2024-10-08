use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use dirs::home_dir;
use eyre::{ErrReport, OptionExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    api_key: String,
}

#[derive(Debug)]
pub enum ConfigError {
    ConfigNotFound,
    ApiKeyNotFound,
    InvalidConfig,
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    Custom(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::ConfigNotFound => {
                write!(f, "Config file not found. Use 'configure' to set up your API key.")
            }
            ConfigError::ApiKeyNotFound => {
                write!(f, "API key not found in config. Use 'configure' to set up your API key.")
            }
            ConfigError::InvalidConfig => write!(
                f,
                "Config file is empty or invalid. Use 'configure' to set up your API key."
            ),
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::JsonError(e) => write!(f, "JSON error: {}", e),
            ConfigError::Custom(e) => write!(f, "Error: {}", e),
        }
    }
}

impl Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::JsonError(error)
    }
}

impl From<ErrReport> for ConfigError {
    fn from(value: ErrReport) -> Self {
        Self::Custom(value.to_string())
    }
}

impl Config {
    fn config_path() -> eyre::Result<PathBuf> {
        Ok(home_dir()
            .ok_or_eyre("home dir not found")?
            .join(".config")
            .join("ghost")
            .join("config.json"))
    }

    fn ensure_get_config() -> Result<PathBuf, ConfigError> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(path)
    }

    fn load() -> Result<Self, ConfigError> {
        let config_file = Self::ensure_get_config()?;
        if config_file.exists() {
            let config_str = fs::read_to_string(&config_file)?;
            if config_str.trim().is_empty() {
                return Err(ConfigError::InvalidConfig);
            }
            match serde_json::from_str(&config_str) {
                Ok(config) => Ok(config),
                Err(_) => Err(ConfigError::InvalidConfig),
            }
        } else {
            Err(ConfigError::ConfigNotFound)
        }
    }

    fn save(&self) -> Result<(), ConfigError> {
        let config_file = Self::ensure_get_config()?;
        let config_str = serde_json::to_string(self)?;
        fs::write(config_file, config_str)?;
        Ok(())
    }
}

pub fn set_api_key(api_key: &str) -> Result<(), ConfigError> {
    let config = Config { api_key: api_key.to_string() };
    config.save()?;
    println!("API key saved successfully in {:?}", Config::config_path());
    Ok(())
}

pub fn get_api_key() -> Result<String, ConfigError> {
    let config = Config::load()?;
    if config.api_key.trim().is_empty() {
        Err(ConfigError::ApiKeyNotFound)
    } else {
        Ok(config.api_key)
    }
}
