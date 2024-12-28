// References to submodules
pub mod env;
pub mod err;
pub mod prelude;
pub mod strings;

// Imports of local modules
use env::constants::RequiredEnvVar;
use env::constants::FILE_PATH as ENV_FILE_PATH;
use env::constants::PREFIX as ENV_PREFIX;
use env::validator::EnvVar;
use err::AppError;

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
    env::load(ENV_FILE_PATH, ENV_PREFIX, RequiredEnvVar::all())?;

    // let var_name = get_var_name(EnvVarName::DbHost);

    println!("Var name: {}", RequiredEnvVar::DbHost.name());
    println!("Var type: {:?}", RequiredEnvVar::DbHost.type_());
    println!("Var value: {:?}", RequiredEnvVar::DbHost.value());

    // todo: build pool connection
    // todo: start http server

    Ok(())
}
