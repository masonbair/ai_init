//! Configuration file handling for ai-init.
//!
//! Loads system-wide configuration from ~/.config/ai-init/config.toml

use crate::types::ProjectType;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Could not determine config directory")]
    NoConfigDir,
}

/// System-wide configuration for ai-init.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub paths: PathsConfig,
    #[serde(default)]
    pub tools: ToolsConfig,
    #[serde(default)]
    pub generation: GenerationConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: DefaultsConfig::default(),
            paths: PathsConfig::default(),
            tools: ToolsConfig::default(),
            generation: GenerationConfig::default(),
        }
    }
}

/// Default behavior settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_true")]
    pub git_init: bool,
    #[serde(default = "default_true")]
    pub create_readme: bool,
    #[serde(default)]
    pub initial_commit: bool,
    #[serde(default)]
    pub project_type: ProjectType,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            git_init: true,
            create_readme: true,
            initial_commit: false,
            project_type: ProjectType::Mixed,
        }
    }
}

/// Path configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PathsConfig {
    pub templates_dir: Option<PathBuf>,
}

/// Custom tool paths (if not in PATH).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolsConfig {
    #[serde(default)]
    pub custom_paths: HashMap<String, PathBuf>,
}

/// Generation options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    #[serde(default = "default_true")]
    pub include_tool_registry: bool,
    #[serde(default = "default_true")]
    pub include_architecture_template: bool,
    #[serde(default = "default_true")]
    pub include_conventions_template: bool,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            include_tool_registry: true,
            include_architecture_template: true,
            include_conventions_template: true,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Config {
    /// Get the default config file path.
    pub fn default_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "ai-tools", "ai-init")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Load configuration from the default location.
    /// Returns default config if file doesn't exist.
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::default_path().ok_or(ConfigError::NoConfigDir)?;

        if !path.exists() {
            return Ok(Self::default());
        }

        Self::load_from(&path)
    }

    /// Load configuration from a specific path.
    pub fn load_from(path: &PathBuf) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save configuration to the default location.
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), ConfigError> {
        let path = Self::default_path().ok_or(ConfigError::NoConfigDir)?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        std::fs::write(&path, contents)?;
        Ok(())
    }

    /// Create a default config file if it doesn't exist.
    #[allow(dead_code)]
    pub fn create_default_if_missing() -> Result<bool, ConfigError> {
        let path = Self::default_path().ok_or(ConfigError::NoConfigDir)?;

        if path.exists() {
            return Ok(false);
        }

        let config = Self::default();
        config.save()?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.defaults.git_init);
        assert!(config.defaults.create_readme);
        assert!(!config.defaults.initial_commit);
    }

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
[defaults]
git_init = false
create_readme = true
initial_commit = true

[tools]
custom_paths = { "code-summarizer" = "/custom/path/code-summarizer" }

[generation]
include_tool_registry = true
"#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(!config.defaults.git_init);
        assert!(config.defaults.create_readme);
        assert!(config.defaults.initial_commit);
        assert!(config.tools.custom_paths.contains_key("code-summarizer"));
    }
}
