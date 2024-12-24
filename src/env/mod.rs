//! This module handles environment realted tasks.
//!
//! It contains its own "constants" submodule where all
//! the environment constants are stored like "ENV_PREFIX"
//! and "ENV_VARS". It also contains a "load" function
//! that loads environment variables from a file and
//! validates them against specified environment variables.

// References to submodules
pub mod constants;
mod validator;

// std library imports
use std::{collections::HashMap, error, fmt};

// Local imports
use crate::{
    err::{AppError, ErrorKind},
    prelude::is_u16,
};
use validator::validate_all;

// * DONE
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
#[derive(Debug, Clone, Copy)]
pub struct EnvVar<'a> {
    pub name: &'a str,
    pub val_type: EnvVarType,
    pub val_example: &'a str,
}

// * DONE
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

// * DONE
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
#[derive(Debug, Clone, Copy)]
pub enum EnvVarType {
    String,
    U16,
}

impl EnvVarType {
    pub fn verify(&self, value: &str) -> Result<(), AppError> {
        match self {
            EnvVarType::String => match value.is_empty() {
                true => {
                    let kind: ErrorKind = ErrorKind::Env;
                    let message: String = format!("Value cannot be empty for type: {:?}", self);
                    let source: Option<Box<dyn error::Error>> = None;

                    Err(AppError::new(kind, message, source))
                }
                false => Ok(()),
            },
            EnvVarType::U16 => match is_u16(value) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }
}

// todo: Implement tests for this function
/// Handles load and validation of application environment.
///
/// Function loads environment file contents at specified
/// path by calling "load_file" function, then if file
/// is valid it will validate loaded environment variables
/// against specified array of environment variables by
/// calling "validate_all" function.
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
/// // todo: assert the result of this function
/// load(file_path, var_prefix, vars);
/// ```
///
/// # Parameters
/// - `file_path`: Path to environment file to load.
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment vriable names to compare against loaded environment.
///
/// # Returns
// todo: Return type, should return a hashmap of environment variables.
pub fn load(
    file_path: &str,
    var_prefix: &str,
    vars: &[EnvVar],
) -> Result<HashMap<String, String>, AppError> {
    // Load environment file contents into std::env
    load_file(file_path)?;

    // Validate loaded environment variables against passed in variables
    // with the specified prefix
    let app_vars: HashMap<String, String> = validate_all(var_prefix, vars)?;

    Ok(app_vars)
}

// * DONE
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
        Ok(_) => Ok(()), // Return Ok if file is loaded successfully
        Err(e) => {
            let kind: ErrorKind = ErrorKind::Env;
            let message: String = format!(
                "Failed to load environment file at specified path: '{}'",
                file_path
            );
            let source: Option<Box<dyn error::Error>> =
                Some(Box::new(e) as Box<dyn std::error::Error>);

            // Return AppError if an error occurs while loading the file
            Err(AppError::new(kind, message, source))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

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

    /// Tests that "load_file" function loads environment file correctly.
    #[test]
    fn test_load_file_valid() {
        // Create a temp file
        let mut temp_file: NamedTempFile =
            NamedTempFile::new().expect("Failed to create temp file");

        // Write some environment variables to the file
        let content: &str = "TEST_VAR=example_value\nANOTHER_VAR=42";
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write to temp file");

        // Get the file path
        let file_path: &str = temp_file.path().to_str().expect("Failed to get file path");

        let result: Result<(), AppError> = load_file(file_path);

        // Assert that the function succeeded
        assert!(result.is_ok(), "load_file failed: {:?}", result.err());
    }

    /// Tests that "load_file" function returns an error if file is not found.
    #[test]
    fn test_load_file_not_found() {
        let file_path: &str = "non_existent_file.env";

        let result: Result<(), AppError> = load_file(file_path);

        // Assert that the function failed
        assert!(result.is_err(), "load_file succeeded: {:?}", result.ok());
    }

    /// Test that "load_file" function returns an error if file is invalid.
    #[test]
    fn test_load_file_invalid() {
        // Create a temp file
        let mut temp_file: NamedTempFile =
            NamedTempFile::new().expect("Failed to create temp file");

        // Write some invalid content to the file
        let content: &str = "TEST_VAR=example_value\nANOTHER_VAR";
        temp_file
            .write_all(content.as_bytes())
            .expect("Failed to write to temp file");

        // Get the file path
        let file_path: &str = temp_file.path().to_str().expect("Failed to get file path");

        let result: Result<(), AppError> = load_file(file_path);

        // Assert that the function failed
        assert!(result.is_err(), "load_file succeeded: {:?}", result.ok());
    }
}
