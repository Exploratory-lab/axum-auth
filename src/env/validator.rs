//! Module that contains functions for validating
//! loaded environment variables.

// Importing external crates
use std::collections::{HashMap, HashSet};

// Importing local modules
use super::{EnvVar, EnvVarType};
use crate::err::{AppError, ErrorKind};

/// ## Validates loaded environment variables.
///
/// Function validates loaded environment variables
/// against specified array of environment variables.
///
/// ## Examples
/// ```
/// use std::collections::HashMap;
/// use axum_auth::env::{validator::validate, EnvVar, EnvVarType};
///
/// const PREFIX: &str = "APP_";
///
/// const REQUIRED_VARS: [EnvVar; 1] = [
///    EnvVar {
///       name: "DB_NAME",
///       val_type: EnvVarType::String
///     },
/// ];
///
/// let loaded_vars: HashMap<String, String> = HashMap::from([
///   ("APP_DB_NAME".to_string(), "my_db".to_string()),
///   ("OTHER_DB_PORT".to_string(), "5432".to_string()),
/// ]);
///
/// let result = validate(PREFIX, &REQUIRED_VARS, loaded_vars);
///
/// let expected: HashMap<&str, String> = HashMap::from([
///  ("DB_NAME", "my_db".to_string()),
/// ]);
///
/// assert_eq!(result.unwrap(), expected);
/// ```
///
///
/// ## Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment variables to validate.
/// - `loaded_vars`: Loaded environment variables.
///
/// ## Returns
/// + `Result<HashMap<&str, String>, AppError>`
///     - `HashMap<&str, String>`: HashMap of application
/// environment variables `<key, value>`.
///     - `AppError`: Error type that contains error kind,
/// message and source.
pub fn validate(
    var_prefix: &str,
    required_vars: HashSet<EnvVar>,
    loaded_vars: HashMap<String, String>,
) -> Result<(), AppError> {
    // Append required variable names with the specified prefix
    let required_vars_with_prefix: HashMap<String, &EnvVar> =
        required_vars.iter().map(|var| (var.name(), var)).collect();

    // Collect variables from the loaded environment that start
    // with the specified prefix and are specified in the
    // 'required_vars_with_prefix'
    let loaded_vars_with_prefix: HashMap<String, String> = loaded_vars
        .into_iter()
        .filter(|(key, _)| {
            key.starts_with(var_prefix) && required_vars_with_prefix.contains_key(key)
        })
        .collect();

    // Will throw an error and stop execution if any of the required
    // variables are missing from the loaded environment, therefore
    // both 'loaded_vars_with_prefix' and 'required_vars_with_prefix'
    // will have the same keys
    check_missing(&loaded_vars_with_prefix, &required_vars_with_prefix)?;

    // Will throw an error and stop execution if any of the required
    // variables have incorrect types, doesn't need 'loaded_vars_with_prefix'
    // as it will access env variable values from the loaded environment
    EnvVar::verify_all()?;

    Ok(())
}

/// ## Checks for missing environment variables.
///
/// Function checks if any of the required environment
/// variables are missing from the loaded environment
/// variables.
///
/// ## Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `required_vars`: Required environment variables.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If no missing variables are found.
///     - `AppError`: If missing variables are found.
fn check_missing(
    loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, &EnvVar>,
) -> Result<(), AppError> {
    let missing_vars: Vec<&str> = required_vars
        .iter()
        .filter_map(|(name, _)| {
            if !loaded_vars.contains_key(name) {
                Some(name.as_str())
            } else {
                None
            }
        })
        .collect();

    if !missing_vars.is_empty() {
        let kind: ErrorKind = ErrorKind::Env;
        let message: String = format!(
            "Missing environment variables: '{}'",
            missing_vars.join(", ")
        );
        let source = None;

        return Err(AppError {
            kind,
            message,
            source,
        });
    }

    Ok(())
}

/// ## Verifies the types of environment variables.
///
/// Function verifies the types of loaded environment
/// variables against the specified types in the required
/// environment variables.
///
/// ## Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `required_vars`: Required environment variables.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If all types are correct.
///     - `AppError`: If any type is incorrect.
fn verify_types(
    // loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, &EnvVar>,
) -> Result<(), AppError> {
    for (_, var_data) in required_vars {
        var_data.verify()?;

        // let value: &String = loaded_vars
        //     .get(name)
        //     .expect("Variable not found during type verification");
        // let var_type: &EnvVarType = &var_data.type_();

        // var_type.verify(value)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    const PREFIX: &str = "APP_";

    // Test `validate`function when all
    // required variables are present, have
    // correct types and are prefixed.
    #[test]
    fn test_validate_all_present() {
        // const REQUIRED_VARS: [EnvVar; 2] = [
        //     EnvVar {
        //         name: "DB_NAME",
        //         val_type: EnvVarType::String,
        //     },
        //     EnvVar {
        //         name: "DB_PORT",
        //         val_type: EnvVarType::U16,
        //     },
        // ];

        // let loaded_vars: HashMap<String, String> = HashMap::from([
        //     ("APP_DB_NAME".to_string(), "my_db".to_string()),
        //     ("APP_DB_PORT".to_string(), "5432".to_string()),
        // ]);

        // let result = validate(PREFIX, &REQUIRED_VARS, loaded_vars);

        // let expected: HashMap<&str, String> = HashMap::from([
        //     ("DB_NAME", "my_db".to_string()),
        //     ("DB_PORT", "5432".to_string()),
        // ]);

        // assert_eq!(
        //     result.unwrap(),
        //     expected,
        //     "validate function failed when it was expected to pass."
        // );
    }

    // Test `validate`function when some required
    // variables are missing i.e. they are not defined
    // or they dont have the correct prefix.
    #[test]
    fn test_validate_missing() {
        //     const REQUIRED_VARS: [EnvVar; 3] = [
        //         EnvVar {
        //             name: "DB_NAME",
        //             val_type: EnvVarType::String,
        //         },
        //         EnvVar {
        //             name: "DB_PORT",
        //             val_type: EnvVarType::U16,
        //         },
        //         EnvVar {
        //             name: "MISSING",
        //             val_type: EnvVarType::String,
        //         },
        //     ];

        //     let loaded_vars: HashMap<String, String> = HashMap::from([
        //         ("APP_DB_NAME".to_string(), "my_db".to_string()),
        //         ("OTHER_DB_PORT".to_string(), "5432".to_string()),
        //     ]);

        //     let result = validate(PREFIX, &REQUIRED_VARS, loaded_vars);

        //     let expected_kind = ErrorKind::Env;
        //     let expected_message = "Missing environment variables: 'DB_PORT, MISSING'".to_string();
        //     let expected_source = None::<Box<dyn error::Error + 'static>>;

        //     let expected_err = AppError {
        //         kind: expected_kind,
        //         message: expected_message,
        //         source: expected_source,
        //     };

        //     assert_eq!(
        //         result.unwrap_err(),
        //         expected_err,
        //         "validate function succeeded when it was expected to fail."
        //     );
    }
}
