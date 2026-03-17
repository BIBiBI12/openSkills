use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub environments: Vec<String>,
    pub version: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            environments: Vec::new(),
            version: "0.1.0".to_string(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(path)
            .context("Failed to read config file")?;
        
        serde_json::from_str(&content)
            .context("Failed to parse config file")
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(path, content)
            .context("Failed to write config file")
    }

    pub fn add_environment(&mut self, env: &str) {
        if !self.environments.contains(&env.to_string()) {
            self.environments.push(env.to_string());
        }
    }

    pub fn remove_environment(&mut self, env: &str) {
        self.environments.retain(|e| e != env);
    }
}
