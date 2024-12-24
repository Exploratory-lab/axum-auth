// References to submodules
pub mod env;
pub mod err;
pub mod prelude;
pub mod strings;

// std imports
use std::collections::HashMap;

// Imports of local modules
use env::constants::FILE_PATH as ENV_FILE_PATH;
use env::constants::PREFIX as ENV_PREFIX;
use env::constants::VARS as ENV_VARS;
use err::AppError;

// todo: Create integration tests for this function
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
/// - `()`: If the function runs successfully.
/// - `AppError`: If the function fails to run.
pub async fn run_app() -> Result<(), AppError> {
    let _app_vars: HashMap<&str, String> = env::load(ENV_FILE_PATH, ENV_PREFIX, &ENV_VARS)?;

    // todo: build pool connection
    // todo: start http server

    Ok(())
}
