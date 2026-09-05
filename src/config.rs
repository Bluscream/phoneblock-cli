use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub default_prefix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "https://phoneblock.net/phoneblock/api".to_string(),
            token: None,
            username: None,
            password: None,
            default_prefix: "+49".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not determine config directory")?;
        path.push("phoneblock");
        fs::create_dir_all(&path)?;
        path.push("config.json");
        Ok(path)
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&content).unwrap_or_default();
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
