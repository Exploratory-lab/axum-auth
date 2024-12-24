//! This module handles environment realted tasks.
//!
//! It contains its own "constants" submodule where all
//! the environment constants are stored like "ENV_PREFIX"
//! and "VARS". It also contains a "load" function
//! that loads environment variables from a file and
//! validates them against specified environment variables.

// References to submodules
pub mod constants;
pub mod validator;

// Importing external crates
use std::{collections::HashMap, error, fmt};

// Importing local modules
use crate::{
    err::{AppError, ErrorKind},
    prelude::is_u16,
};
use validator::validate;

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
/// Enum represents the type of the environment variables.
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
/// - `PgSslMode`: Postgres SSL mode type environment variable.
#[derive(Debug, Clone, Copy)]
pub enum EnvVarType {
    String,
    U16,
    Enum(&'static [&'static str]),
}

/// Implementation block for "EnvVarType" enum.
impl EnvVarType {
    const ERR_KIND: ErrorKind = ErrorKind::Env;

    /// Verifies the value of the environment variable.
    ///
    /// Function verifies the value of the environment variable
    /// against the type of the environment variable.
    ///
    /// # Examples
    /// ```
    /// use axum_auth::env::EnvVarType;
    ///
    /// let var_type = EnvVarType::String;
    /// let value = "example_value";
    ///
    /// assert!(var_type.verify(value).is_ok());
    /// ```
    ///
    /// # Parameters
    /// - `value`: Value of the environment variable to verify.
    ///
    /// # Returns
    /// - `()`: If value is valid for the environment variable type.
    /// - `AppError`: Error type that contains error kind,
    /// message and source.
    pub fn verify(&self, value: &str) -> Result<(), AppError> {
        match self {
            EnvVarType::String => match value.is_empty() {
                true => {
                    let message: String = format!("Value cannot be empty for type: {:?}", self);
                    let source: Option<Box<dyn error::Error>> = None;

                    Err(AppError::new(Self::ERR_KIND, message, source))
                }
                false => Ok(()),
            },
            EnvVarType::U16 => match is_u16(value, Some(Self::ERR_KIND)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            EnvVarType::Enum(allowed_values) => {
                if allowed_values.contains(&value) {
                    Ok(())
                } else {
                    let message: String =
                        format!("Value '{}' is not allowed for type {:?}", value, self);
                    let source: Option<Box<dyn error::Error>> = None;

                    Err(AppError::new(Self::ERR_KIND, message, source))
                }
            }
        }
    }
}

/// Handles load and validation of application environment.
///
/// Function loads environment file contents at specified
/// path by calling "load_file" function, then if file
/// is valid it will validate loaded environment variables
/// against specified array of environment variables by
/// calling "validate" function.
///
/// # Examples
/// ```
/// use axum_auth::env::{EnvVar, EnvVarType, load};
/// use tempfile::NamedTempFile;
/// use std::io::Write;
/// use std::collections::HashMap;
///
/// // Create a temp file
/// let mut temp_file: NamedTempFile = NamedTempFile::new().expect("Failed to create temp file");
///
/// // Write some environment variables to the file
/// let content: &str = "APP_TEST_VAR=example_value\nANOTHER_VAR=42";
/// temp_file.write_all(content.as_bytes()).expect("Failed to write to temp file");
///
/// // Get the file path
/// let file_path: &str = temp_file.path().to_str().expect("Failed to get file path");
///
/// let var_prefix: &str = "APP_";
///
/// // Define required environment variables
/// const REQUIRED_VARS: [EnvVar; 1] = [
///    EnvVar { name: "TEST_VAR", val_type: EnvVarType::String, val_example: "example_value" },
///    ];
///
/// // Load and validate environment variables
/// let result = load(file_path, var_prefix, &REQUIRED_VARS);
///
/// eprintln!("{:?}", result);
///
/// let expected: HashMap<&str, String> = HashMap::from([("TEST_VAR", "example_value".to_string())]);
///
/// assert_eq!(result.unwrap(), expected);
/// ```
///
/// # Parameters
/// - `file_path`: Path to environment file to load.
/// - `var_prefix`: Prefix for environment variables.
/// - `required_vars`: Array of environment vriable names
/// to compare against loaded environment.
///
/// # Returns
/// - `HashMap<&str, String>`: HashMap of application
/// environment variables <key, value>.
/// - `AppError`: Error type that contains error kind,
/// message and source.
pub fn load<'a>(
    file_path: &str,
    var_prefix: &str,
    required_vars: &'a [EnvVar],
) -> Result<HashMap<&'a str, String>, AppError>
where
    'a: 'static,
{
    // Load environment file contents into std::env
    load_file(file_path)?;

    let loaded_vars: HashMap<String, String> = std::env::vars().collect();

    // Validate loaded environment variables against
    // passed in variables with the specified prefix
    let app_vars: HashMap<&str, String> = validate(var_prefix, required_vars, loaded_vars)?;

    Ok(app_vars)
}

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
/// - `()`: If file is loaded successfully.
/// - `AppError`: Error type that contains error kind,
/// message and source.
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

    /// Tests that "verify" function verifies
    /// the value of the environment variable correctly
    /// for "String" variant.
    #[test]
    fn test_verify_string() {
        let var_type: EnvVarType = EnvVarType::String;
        let value: &str = "example_value";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function succeeded
        assert!(result.is_ok(), "verify failed: {:?}", result.err());
    }

    /// Tests that "verify" with "String" variant returns an error
    /// if the value is empty.
    #[test]
    fn test_verify_string_empty() {
        let var_type: EnvVarType = EnvVarType::String;
        let value: &str = "";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function failed
        assert!(result.is_err(), "verify succeeded: {:?}", result.ok());
    }

    /// Tests that "verify" function verifies
    /// the value of the environment variable correctly
    /// for "U16" variant.
    #[test]
    fn test_verify_u16() {
        let var_type: EnvVarType = EnvVarType::U16;
        let value: &str = "42";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function succeeded
        assert!(result.is_ok(), "verify failed: {:?}", result.err());
    }

    /// Tests that "verify" with "U16" variant returns an error
    /// if the value is not a valid u16.
    #[test]
    fn test_verify_u16_invalid() {
        let var_type: EnvVarType = EnvVarType::U16;
        let value: &str = "invalid";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function failed
        assert!(result.is_err(), "verify succeeded: {:?}", result.ok());
    }

    /// Tests that "verify" function verifies
    /// the value of the environment variable correctly
    /// for "Enum" variant.
    #[test]
    fn test_verify_enum() {
        let allowed_values: &'static [&'static str] = &["value1", "value2"];
        let var_type: EnvVarType = EnvVarType::Enum(allowed_values);
        let value: &str = "value1";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function succeeded
        assert!(result.is_ok(), "verify failed: {:?}", result.err());
    }

    /// Tests that "verify" with "Enum" variant returns an error
    /// if the value is not allowed.
    #[test]
    fn test_verify_enum_not_allowed() {
        let allowed_values: &'static [&'static str] = &["value1", "value2"];
        let var_type: EnvVarType = EnvVarType::Enum(allowed_values);
        let value: &str = "value3";

        let result: Result<(), AppError> = var_type.verify(value);

        // Assert that the function failed
        assert!(result.is_err(), "verify succeeded: {:?}", result.ok());
    }
}
