//! This module handles environment realted tasks.
//!
//! It contains its own "constants" submodule where all
//! the environment constants are stored like "ENV_PREFIX"
//! and "ENV_VARS". It also contains a "load" function
//! that loads environment variables from a file and
//! validates them against specified environment variables.

// References to submodules
pub mod constants;

// std library imports
use std::{error, fmt};

// Local imports
use crate::err::{AppError, ErrorKind};

/// Environment variable struct.
///
/// Struct represents an environment variable with
/// its name, type and example value.
///
/// # Examples
/// ```
/// use axum_auth::env::{EnvVar, EnvVarType};
///
/// let var = EnvVar { name: "VAR_NAME",
///                    val_type: EnvVarType::String,
///                    val_example: "example_value"
///                   };
/// ```
///
/// # Fields
/// - `name`: Name of the environment variable.
/// - `val_type`: Type of the environment variable.
/// - `val_example`: Example value for the environment variable.
pub struct EnvVar<'a> {
    pub name: &'a str,
    pub val_type: EnvVarType,
    pub val_example: &'a str,
}

/// Implements "Display" trait for "EnvVar" struct.
///
/// Trait formats "EnvVar" struct for display.
///
/// # Examples
/// ```
/// use axum_auth::env::{EnvVar, EnvVarType};
///
/// let var = EnvVar { name: "VAR_NAME",
///                    val_type: EnvVarType::String,
///                    val_example: "example_value"
///                  };
/// let expected: &str = "EnvVar { name: VAR_NAME, val_type: String, val_example: example_value }";
///
/// assert_eq!(format!("{}", var), expected);
/// ```
impl fmt::Display for EnvVar<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "EnvVar {{ name: {}, val_type: {:?}, val_example: {} }}",
            self.name, self.val_type, self.val_example
        )
    }
}

/// Environment variable type enum.
///
/// Enum represents the type of the environment variable.
///
/// # Examples
/// ```
/// use axum_auth::env::EnvVarType;
///
/// let var_type = EnvVarType::String;
/// ```
///
/// # Variants
/// - `String`: String type environment variable.
/// - `U16`: Unsigned 16-bit integer type environment variable.
#[derive(Debug)]
pub enum EnvVarType {
    String,
    U16,
}

// TODO: Implement tests for this function
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
/// use axum_auth::env::{EnvVar, EnvVarType, load};
///
/// let file_path = ".env";
/// let var_prefix = "EX_";
/// let vars = &[EnvVar { name: "VAR_NAME",
///                       val_type: EnvVarType::String,
///                       val_example: "example_value"
///                     }];
/// // TODO: assert the result of this function
/// load(file_path, var_prefix, vars);
/// ```
///
/// # Parameters
/// - `file_path`: Path to environment file to load.
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment vriable names to compare against loaded environment.
///
/// # Returns
/// - TODO: Return type, should return a hashmap of environment variables.
pub fn load(file_path: &str, var_prefix: &str, vars: &[EnvVar]) -> Result<(), AppError> {
    load_file(file_path)?;

    validate_env(var_prefix, vars);

    Ok(())
}

// TODO: Implement tests for this function
/// Loads environment file contents (private).
///
/// Function uses "from_filename" function from "dotenvy"
/// crate in order to load environment variables from
/// file at the specified file path.
///
/// # Parameters
/// -  `file_path`: Path to environment file to load.
///
/// # Returns
/// - `Ok(())` if the file is loaded successfully.
/// - `Err(AppError)` if an error occurs.
fn load_file(file_path: &str) -> Result<(), AppError> {
    match dotenvy::from_filename(file_path) {
        Ok(_) => Ok(()),
        Err(e) => {
            let kind: ErrorKind = ErrorKind::Env;
            let message: String = format!(
                "Failed to load environment file at specified path: '{}'",
                file_path
            );
            let source: Option<Box<dyn error::Error>> =
                Some(Box::new(e) as Box<dyn std::error::Error>);

            Err(AppError::new(kind, message, source))
        }
    }
}

// TODO: Implement tests for this function
/// Validates loaded environment variables (private).
///
/// Function validates loaded environment variables
/// against specified array of environment variables.
///
/// # Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment variables to validate.
///
/// # Returns
/// - TODO: Return type, should return a hashmap of environment variables.
fn validate_env(var_prefix: &str, vars: &[EnvVar]) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that "EnvVar" struct is displayed correctly.
    #[test]
    fn test_env_var_display() {
        let var: EnvVar<'_> = EnvVar {
            name: "VAR_NAME",
            val_type: EnvVarType::String,
            val_example: "example_value",
        };

        let expected: &str =
            "EnvVar { name: VAR_NAME, val_type: String, val_example: example_value }";

        assert_eq!(format!("{}", var), expected);
    }
}
