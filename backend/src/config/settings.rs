use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
    pub upload: UploadSettings,
    pub notification: NotificationSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UploadSettings {
    pub max_size_mb: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationSettings {
    pub event_batch_size: u32,
    pub delivery_batch_size: u32,
    pub max_attempts: u32,
    pub retention_days: u32,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let config_path = manifest_dir.join("config/default.toml");
        let env_path = manifest_dir.join(".env");

        let _ = dotenvy::from_path(env_path);

        Config::builder()
            .add_source(File::from(config_path))
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }
}