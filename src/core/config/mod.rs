use config::{Config, ConfigError};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static APP_CONFIG: Lazy<AppConfig> =
    Lazy::new(|| load_config().expect("Failed to load configuration"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub env: String,
    pub prefix: String,
    pub env_file_path: String,
}

fn load_config() -> Result<AppConfig, ConfigError> {
    // Load configuration from config.toml
    let app_config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    // Deserialize into the AppConfig struct
    app_config.try_deserialize::<AppConfig>()
}
