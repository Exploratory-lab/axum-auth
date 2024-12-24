//! Module that contains functions for validating
//! loaded environment variables.

// Importing external crates
use std::collections::HashMap;

// Importing local modules
use super::{EnvVar, EnvVarType};
use crate::err::{AppError, ErrorKind};

/// Validates loaded environment variables.
///
/// Function validates loaded environment variables
/// against specified array of environment variables.
///
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use axum_auth::env::{validator::validate, EnvVar, EnvVarType};
///
/// const PREFIX: &str = "APP_";
///
/// const REQUIRED_VARS: [EnvVar; 1] = [
///    EnvVar {
///       name: "DB_NAME",
///       val_type: EnvVarType::String,
///       val_example: "my_db",
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
/// # Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars`: Array of environment variables to validate.
/// - `loaded_vars`: Loaded environment variables.
///
/// # Returns
/// - `HashMap<&str, String>`: HashMap of application
/// environment variables <key, value>
/// - `AppError`: Error type that contains error kind,
/// message and source.
pub fn validate<'a>(
    var_prefix: &str,
    vars: &'a [EnvVar],
    loaded_vars: HashMap<String, String>,
) -> Result<HashMap<&'a str, String>, AppError>
where
    'a: 'static,
{
    // todo: use logger service here
    // Print warning if the 'VARS' array in 'env.rs' is not up to date
    // with the environment variables in '.env' file
    println!("Warning: make sure 'VARS' array in 'axum_auth::env::constants' is up to date with the environment variables in '.env' file.");

    // Append required variable names with the specified prefix
    let required_vars_with_prefix: HashMap<String, &EnvVar> = vars
        .iter()
        .map(|var| (format!("{}{}", var_prefix, var.name), var))
        .collect();

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

    verify_types(&loaded_vars_with_prefix, &required_vars_with_prefix)?;

    let constructed_vars: HashMap<&str, String> =
        construct_variables(loaded_vars_with_prefix, required_vars_with_prefix)?;

    Ok(constructed_vars)
}

/// Checks for missing environment variables.
///
/// Function checks if any of the required environment
/// variables are missing from the loaded environment
/// variables.
///
/// # Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `required_vars`: Required environment variables.
///
/// # Returns
/// - `()`: If no missing variables are found.
/// - `AppError`: If missing variables are found.
fn check_missing(
    loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, &EnvVar>,
) -> Result<(), AppError> {
    let missing_vars: Vec<&EnvVar> = required_vars
        .iter()
        .filter_map(|(name, var_data)| {
            if !loaded_vars.contains_key(name) {
                Some(*var_data)
            } else {
                None
            }
        })
        .collect();

    if !missing_vars.is_empty() {
        let missing_var_names: Vec<&str> = missing_vars.iter().map(|var| var.name).collect();

        let kind: ErrorKind = ErrorKind::Env;
        let message: String = format!(
            "Missing environment variables: '{}'",
            missing_var_names.join(", ")
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

/// Verifies the types of environment variables.
///
/// Function verifies the types of loaded environment
/// variables against the specified types in the required
/// environment variables.
///
/// # Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `required_vars`: Required environment variables.
///
/// # Returns
/// - `()`: If all types are correct.
/// - `AppError`: If any type is incorrect.
fn verify_types(
    loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, &EnvVar>,
) -> Result<(), AppError> {
    for (name, var_data) in required_vars {
        let value: &String = loaded_vars
            .get(name)
            .expect("Variable not found during type verification");
        let var_type: &EnvVarType = &var_data.val_type;

        var_type.verify(value)?;
    }

    Ok(())
}

/// Constructs application environment variables.
///
/// Function constructs application environment variables
/// from the loaded environment variables.
///
/// # Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `required_vars`: Required environment variables.
///
/// # Returns
/// - `HashMap<&str, String>`: HashMap of application
/// environment variables <key, value>.
/// - `AppError`: Error type that contains error kind,
/// message and source.
fn construct_variables<'a>(
    mut loaded_vars: HashMap<String, String>,
    required_vars: HashMap<String, &EnvVar<'a>>,
) -> Result<HashMap<&'a str, String>, AppError>
where
    'a: 'static,
{
    let mut constructed_vars: HashMap<&str, String> = HashMap::new();

    for (name, var_data) in required_vars {
        let value = loaded_vars
            .remove(&name)
            .expect("Variable missing during construction despite prior validation");
        constructed_vars.insert(var_data.name, value);
    }

    Ok(constructed_vars)
}
