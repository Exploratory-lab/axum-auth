//! TODO: Add a description

// References to submodules
mod env;
mod err;

// Imports of local modules
use env::constants::FILE_PATH as ENV_FILE_PATH;
use env::constants::PREFIX as ENV_PREFIX;
use env::constants::VARS as ENV_VARS;

#[tokio::main]
async fn main() {
    run_app().await;
}

/// Runs the application.
///
/// Function loads environment variables from file
/// and validates them against specified environment
/// variables. Then it builds a connection pool and
/// starts the HTTP server.
///
/// # Examples
/// ```
/// run_app();
/// ```
///
/// #Returns
/// - ...
async fn run_app() {
    env::load(ENV_FILE_PATH, ENV_PREFIX, &ENV_VARS);

    // TODO: build pool connection
    // TODO: start http server
}
