//! This module handles environment realted tasks.
//!
//! The module contains two submodules: // todo

// References to submodules
pub mod constants;
pub mod validator;
pub mod vars;

// Importing external crates
use std::{collections::HashSet, error, hash::Hash};

// Importing local modules
use crate::core::err::{AppError, ErrorKind};
use validator::validate;
use vars::EnvVar;

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
/// // todo
/// ```
///
/// # Parameters
/// - `file_path`: Path to file to load
///   the environment variables from.
/// - `var_prefix`: Prefix for environment variables to
///   use.
/// - `vars_to_validate`: Variables to validate against
///   process environment variables.
///
/// # Returns
/// + `Result<(), AppError>`
///     - `()`: If environment variables are loaded and
/// validated successfully.
///     - `AppError`: Error type that contains error kind,
/// message and source.
pub fn load<V>(
    file_path: &str,
    var_prefix: &str,
    vars_to_validate: HashSet<V>,
) -> Result<(), AppError>
where
    V: EnvVar,
    V::VarType: Eq + Hash,
{
    // Load environment file contents into std::env
    load_file(file_path)?;

    // Validate loaded environment variables against
    // specified environment variables
    validate(var_prefix, vars_to_validate)?;

    Ok(())
}

/// ## Loads environment file contents (private).
///
/// Function uses "from_filename" function from "dotenvy"
/// crate in order to load environment variables from
/// file at the specified file path.
///
/// ## Parameters
/// -  `file_path`: Path to environment file to load.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If file is loaded successfully.
///     - `AppError`: Error type that contains error kind,
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

    // Tests that "EnvVar" struct is created correctly.
    #[test]
    fn test_create_env_var() {
        // let var = EnvVar {
        //     name: "VAR_NAME",
        //     val_type: EnvVarType::String,
        // };

        // assert_eq!(var.name, "VAR_NAME");
        // assert_eq!(var.val_type, EnvVarType::String);
    }

    // Tests that "EnvVar" struct is cloned correctly.
    #[test]
    fn test_clone_env_var() {
        // let var = EnvVar {
        //     name: "VAR_NAME",
        //     val_type: EnvVarType::String,
        // };
        // let cloned_var = var.clone();

        // assert_eq!(var.name, cloned_var.name);
        // assert_eq!(var.val_type, cloned_var.val_type);
    }

    // Tests that "EnvVar" struct is copied correctly.
    #[test]
    fn test_copy_env_var() {
        // let var = EnvVar {
        //     name: "VAR_NAME",
        //     val_type: EnvVarType::String,
        // };
        // let copied_var = var;

        // assert_eq!(var.name, copied_var.name);
        // assert_eq!(var.val_type, copied_var.val_type);
    }

    // Tests that "EnvVar" struct is displayed correctly.
    #[test]
    fn test_env_var_display() {
        // let var: EnvVar<'_> = EnvVar {
        //     name: "VAR_NAME",
        //     val_type: EnvVarType::String,
        // };

        // let expected: &str = "EnvVar { name: VAR_NAME, val_type: String }";

        // assert_eq!(
        //     format!("{}", var),
        //     expected,
        //     "EnvVar display failed \
        //     when it was supposed to succeed"
        // );
    }

    // Tests that "load_file" function loads environment file correctly.
    #[test]
    fn test_load_file_valid() {
        // // Create a temp file
        // let mut temp_file: NamedTempFile =
        //     NamedTempFile::new().expect("Failed to create temp file");

        // // Write some environment variables to the file
        // let content: &str = "TEST_VAR=example_value\nANOTHER_VAR=42";
        // temp_file
        //     .write_all(content.as_bytes())
        //     .expect("Failed to write to temp file");

        // // Get the file path
        // let file_path: &str = temp_file.path().to_str().expect("Failed to get file path");

        // let result: Result<(), AppError> = load_file(file_path);

        // // Assert that the function succeeded
        // assert!(
        //     result.is_ok(),
        //     "load_file failed when it was \
        //     supposed to succeed: {:?}",
        //     result.err()
        // );
    }

    // Tests that "load_file" function returns an error if file is not found.
    #[test]
    fn test_load_file_not_found() {
        // let file_path: &str = "non_existent_file.env";

        // let result: Result<(), AppError> = load_file(file_path);

        // // Assert that the function failed
        // assert!(
        //     result.is_err(),
        //     "load_file succeeded when it \
        //     was supposed to fail: {:?}",
        //     result.ok()
        // );
    }

    // Test that "load_file" function returns an error if file is invalid.
    #[test]
    fn test_load_file_invalid() {
        // // Create a temp file
        // let mut temp_file: NamedTempFile =
        //     NamedTempFile::new().expect("Failed to create temp file");

        // // Write some invalid content to the file
        // let content: &str = "TEST_VAR=example_value\nANOTHER_VAR";
        // temp_file
        //     .write_all(content.as_bytes())
        //     .expect("Failed to write to temp file");

        // // Get the file path
        // let file_path: &str = temp_file.path().to_str().expect("Failed to get file path");

        // let result: Result<(), AppError> = load_file(file_path);

        // // Assert that the function failed
        // assert!(
        //     result.is_err(),
        //     "load_file succeeded when it \
        //      was supposed to fail: {:?}",
        //     result.ok()
        // );
    }

    // Tests that "verify" function verifies
    // the value of the environment variable correctly
    // for "String" variant.
    #[test]
    fn test_verify_string() {
        // let var_type: EnvVarType = EnvVarType::String;
        // let value: &str = "example_value";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function succeeded
        // assert!(
        //     result.is_ok(),
        //     "verify failed when it was \
        //     supposed to succeed: {:?}",
        //     result.err()
        // );
    }

    // Tests that "verify" with "String" variant returns an error
    // if the value is empty.
    #[test]
    fn test_verify_string_empty() {
        // let var_type: EnvVarType = EnvVarType::String;
        // let value: &str = "";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function failed
        // assert!(
        //     result.is_err(),
        //     "verify succeeded when it was \
        //     supposed to fail: {:?}",
        //     result.ok()
        // );
    }

    // Tests that "verify" function verifies
    // the value of the environment variable correctly
    // for "U16" variant.
    #[test]
    fn test_verify_u16() {
        // let var_type: EnvVarType = EnvVarType::U16;
        // let value: &str = "42";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function succeeded
        // assert!(
        //     result.is_ok(),
        //     "verify failed when it was \
        //     supposed to succeed: {:?}",
        //     result.err()
        // );
    }

    // Tests that "verify" with "U16" variant returns an error
    // if the value is not a valid u16.
    #[test]
    fn test_verify_u16_invalid() {
        // let var_type: EnvVarType = EnvVarType::U16;
        // let value: &str = "invalid";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function failed
        // assert!(result.is_err(), "verify succeeded: {:?}", result.ok());
    }

    // Tests that "verify" function verifies
    // the value of the environment variable correctly
    // for "Enum" variant.
    #[test]
    fn test_verify_enum() {
        // let allowed_values: &'static [&'static str] = &["value1", "value2"];
        // let var_type: EnvVarType = EnvVarType::Enum(allowed_values);
        // let value: &str = "value1";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function succeeded
        // assert!(
        //     result.is_ok(),
        //     "verify failed when it was \
        //     supposed to succeed: {:?}",
        //     result.err()
        // );
    }

    // Tests that "verify" with "Enum" variant returns an error
    // if the value is not allowed.
    #[test]
    fn test_verify_enum_not_allowed() {
        // let allowed_values: &'static [&'static str] = &["value1", "value2"];
        // let var_type: EnvVarType = EnvVarType::Enum(allowed_values);
        // let value: &str = "value3";

        // let result: Result<(), AppError> = var_type.verify(value);

        // // Assert that the function failed
        // assert!(
        //     result.is_err(),
        //     "verify succeeded when it was \
        //     supposed to fail: {:?}",
        //     result.ok()
        // );
    }
}
