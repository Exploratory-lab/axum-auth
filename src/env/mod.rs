//! This module handles environment realted tasks.
//!
//! It contains its own "constants" submodule where all
//! the environment constants are stored like "ENV_PREFIX"
//! and "ENV_VARS". It also contains a "load" function
//! that loads environment variables from a file and
//! validates them against specified environment variables.

// References to submodules
pub mod constants;

/// Environment variable struct.
///
/// Struct represents an environment variable with
/// its name and optional validator.
///
/// # Examples
/// ```
/// let var = EnvVar { name: "VAR_NAME", validator: None };
/// ```
///
/// # Fields
/// - `name`: Name of the environment variable.
/// - `validator`: Optional validator for the environment variable.
#[derive(Debug)]
pub struct EnvVar<'a> {
    pub name: &'a str,
    pub validator: Option<&'a str>,
}

/// Handles load and validation of application environment.
///
/// Function loads environment file contents at specified
/// path by calling "load_file" function, then if file
/// is valid it will validate loaded environment variables
/// against specified array of environment variables by
/// calling "validate_env" function.
///
/// # Examples
/// ```
/// let file_path = ".env";
/// let var_prefix = "EX_";
/// let vars = &[EnvVar { name: "VAR_NAME", validator: None }];
///
/// load_env(file_path, var_prefix, vars);
/// ```
///
/// # Parameters
/// - `file_path`: Path to environment file to load.
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment vriable names to compare against loaded environment.
///
/// # Returns
/// - TODO: Return type
pub fn load(file_path: &str, var_prefix: &str, vars: &[EnvVar]) {
    load_file(file_path);

    validate_env(var_prefix, vars);
}

/// Loads environment file contents (private).
///
/// Function uses "from_filename" function from "dotenvy"
/// crate in order to load environment variables from
/// file at the specified file path.
///
/// # Examples
/// ```
/// let file_path = ".env";
///
/// load_file(file_path);
/// ```
///
/// # Parameters
/// -  `file_path`: Path to environment file to load.
///
/// # Returns
/// - TODO: Return type
fn load_file(file_path: &str) {
    let load_result: Result<std::path::PathBuf, dotenvy::Error> = dotenvy::from_filename(file_path);
}

/// Validates loaded environment variables (private).
///
/// Function validates loaded environment variables
/// against specified array of environment variables.
///
/// # Examples
/// ```
/// let var_prefix = "EX_";
/// let vars = &[EnvVar { name: "VAR_NAME", validator: None }];
///
/// validate_env(var_prefix, vars);
/// ```
///
/// # Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment variables to validate.
///
/// # Returns
/// - TODO: Return type
fn validate_env(var_prefix: &str, vars: &[EnvVar]) {
    // ...
}
