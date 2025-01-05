//! Application configuration module.
//!
//! Module is responsible for loading,
//! validating and holding the application
//! configuration settings.

// Imports from external crates
use config::Config;
use once_cell::sync::{Lazy, OnceCell};
use serde::Deserialize;

// Local imports
use super::err::{AppError, ErrorKind};

/// Default configuration file name.
pub const DEFAULT_CONFIG_FILE: &str = "./config";

/// Holds the name of the configuration file.
pub static CONFIG_FILE_PATH: OnceCell<String> = OnceCell::new();

/// Application configuration.
pub static APP_CONFIG: Lazy<Option<AppConfig>> = Lazy::new(|| {
    // Get the configuration file path, if it is not set
    // use the default configuration file name
    let config_file = match CONFIG_FILE_PATH.get() {
        Some(file) => file,
        None => &DEFAULT_CONFIG_FILE.to_string(),
    };

    // Load the configuration from the file and return it
    load_config(config_file)
});

/// ## Application configuration struct.
///
/// ## Fields
/// + `app`: `AppSettings` - Application settings.
///
/// ## Examples
/// ```
/// use axum_auth::core::config::AppConfig;
///
/// let app_config = AppConfig {
///    app: AppSettings {
///       env: "development".to_string(),
///       prefix: "APP".to_string(),
///       env_file_path: ".env".to_string(),
///    },
/// };
/// ```
#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub app: AppSettings,
}

/// ## Application settings struct.
///
/// ## Fields
/// + `env`: `String` - Application environment.
/// + `prefix`: `String` - Prefix for environment variables.
/// + `env_file_path`: `String` - Path to the environment file.
///
/// ## Examples
/// ```
/// use axum_auth::core::config::AppSettings;
///
/// let app_settings = AppSettings {
///   env: "development".to_string(),
///   prefix: "APP".to_string(),
///   env_file_path: ".env".to_string(),
/// };
/// ```
#[derive(Debug, Deserialize, PartialEq)]
pub struct AppSettings {
    pub env: String,
    pub prefix: String,
    pub env_file_path: String,
}

/// ## Checks if the configuration was loaded successfully.
///
/// Function checks if the configuration was loaded successfully.
/// Function can be used before accessing the configuration to
/// ensure that it was loaded successfully.
///
/// ## Exaples
/// ```
/// // todo
/// ```
///
/// ## Returns
/// + `Result<(), AppError>`
///    - `Ok(())` - If the configuration was loaded successfully.
///    - `Err(AppError)` - If the configuration was not loaded successfully.
pub fn get_config(
    config_instance: &'static Lazy<Option<AppConfig>>,
) -> Result<&'static AppConfig, AppError> {
    match config_instance.as_ref() {
        Some(config) => Ok(config),
        None => Err(AppError::new(
            ErrorKind::InvalidConfig,
            "Configuration file is missing or failed to load".to_string(),
            None,
        )),
    }
}

/// ## Loads the configuration from the file.
///
/// Function loads the configuration from the
/// specified file name.
///
/// ## Parameters
/// + `file_name`: `&str` - Name of the configuration file.
///
/// ## Returns
/// + `Option<AppConfig>` - Loaded configuration.
///   - `Some(AppConfig)` - If the configuration was loaded successfully.
///   - `None` - If the configuration failed to load/deserialize.
fn load_config(file_name: &str) -> Option<AppConfig> {
    // Load configuration from the file in the current working directory
    let app_config = match build_config_from_file(file_name) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return None;
        }
    };

    // Deserialize into the AppConfig struct
    match app_config.try_deserialize::<AppConfig>() {
        Ok(app_config) => Some(app_config),
        Err(e) => {
            eprintln!("Failed to deserialize configuration: {}", e);
            return None;
        }
    }
}

/// ## Builds the configuration from the file.
///
/// Function builds the configuration from the file
/// specified by the file path.
///
/// ## Parameters
/// + `file_path`: `&str` - Path to the configuration file.
///
/// ## Returns
/// + `Result<Config, AppError>` - Loaded configuration.
///   - `Ok(Config)` - If the configuration was loaded successfully.
///   - `Err(AppError)` - If the configuration failed to load.
fn build_config_from_file(file_path: &str) -> Result<Config, AppError> {
    let app_config = Config::builder()
        .add_source(config::File::with_name(file_path))
        .build()
        .map_err(|e| {
            AppError::new(
                ErrorKind::InvalidConfig,
                format!("Failed to load configuration: {}", e),
                Some(Box::new(e)),
            )
        })?;

    Ok(app_config)
}

#[cfg(test)]
mod tests {}
