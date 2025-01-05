// References to submodules
// pub mod env;
// pub mod err;
pub mod core;
pub mod strings;

// Imports of local modules
use core::config::get_config;
use core::config::APP_CONFIG;
use core::config::CONFIG_FILE_PATH;
use core::env::vars::{EnvVar, RequiredEnvVar};
use core::err::{AppError, ErrorKind};

/// Runs the application.
///
/// Function loads environment variables from file
/// and validates them against specified environment
/// variables. Then it builds a connection pool and
/// starts the HTTP server.
///
/// # Examples
/// ```
/// use axum_auth::run_app;
///
/// run_app();
/// ```
///
/// #Returns
/// + `Result<(), AppError>`
///   - `()`: If the function runs successfully.
///   - `AppError`: If the function fails to run.
pub async fn run_app() -> Result<(), AppError> {
    // * Temporary code
    let config_file_path: String = "./custom_config.toml".to_string();
    // Set the configuration file path
    set_config_file_path(config_file_path)?;

    // Check if the configuration is loaded and
    // if it is valid
    let app_config = get_config(&APP_CONFIG)?;

    println!("App Config: {:?}", app_config);

    // Load environment variables from file
    // core::env::load(
    //     &app_config.app.env_file_path,
    //     &app_config.app.prefix,
    //     RequiredEnvVar::all(),
    // )?;

    // println!("DB_HOST: {}", RequiredEnvVar::DbHost.value());

    Ok(())
}

// * Temporary code
fn set_config_file_path(path: String) -> Result<(), AppError> {
    // !! This is a simulation, this parameter will come
    // !! from the command line arguments
    if let Err(_) = CONFIG_FILE_PATH.set(path) {
        let kind = ErrorKind::ConfigFilePath;
        let message = "Failed to set configuration file path".to_string();
        let source = None;

        let err = AppError::new(kind, message, source);

        return Err(err);
    }

    Ok(())
}
