use std::collections::{HashMap, HashSet};

use crate::err::{AppError, ErrorKind};

use super::{EnvVar, EnvVarType};

// todo: Implement tests for this function
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
// todo: Return type, should return a hashmap of environment variables.
pub fn validate_all(
    var_prefix: &str,
    vars: &[EnvVar],
) -> Result<HashMap<String, String>, AppError> {
    // 1. Filter out environment variables with the specified prefix
    // from the std::env::vars() iterator
    //
    // 2. Compare the filtered environment variables with the
    // specified array of environment variables
    //
    // 3. If any of the specified environment variables are not
    // found in the loaded environment variables, return an error
    // with the missing environment variable names
    //
    // 4. Warn the user if there are loaded environment variables
    // that are not specified in the specified array of environment
    // variables
    //
    // 5. If all specified environment variables are found in the
    // loaded environment variables, validate their types and
    //
    // 6. If any of the loaded environment variables have invalid
    // types, return an error with the invalid environment variable
    // names
    //
    // 7. If all loaded environment variables are valid, return
    // a hashmap of the loaded environment variables

    // Collect variables from the loaded environment that start
    // with the specified prefix
    let loaded_vars_with_prefix: HashMap<String, String> = std::env::vars()
        .filter(|(key, _)| key.starts_with(var_prefix))
        .collect();

    // Append required variable names with the specified prefix
    let required_vars_with_prefix: HashMap<String, EnvVar> = vars
        .iter()
        .map(|var| (format!("{}{}", var_prefix, var.name), *var))
        .collect();

    check_unknown(&loaded_vars_with_prefix, &required_vars_with_prefix);

    check_missing(&loaded_vars_with_prefix, &required_vars_with_prefix)?;

    Ok(HashMap::new())
}

fn check_missing(
    loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, EnvVar>,
) -> Result<(), AppError> {
    let missing_vars: Vec<&EnvVar> = required_vars
        .iter()
        .filter_map(|(name, var_data)| {
            if !loaded_vars.contains_key(name) {
                Some(var_data)
            } else {
                None
            }
        })
        .collect();

    if !missing_vars.is_empty() {
        let kind: ErrorKind = ErrorKind::Env;
        let message: String = format!("Missing environment variables: {:?}", missing_vars);
        let source = None;

        return Err(AppError {
            kind,
            message,
            source,
        });
    }

    Ok(())
}

fn check_unknown(loaded_vars: &HashMap<String, String>, required_vars: &HashMap<String, EnvVar>) {
    let unknown_vars: HashSet<&String> = loaded_vars
        .iter()
        .filter_map(|(name, _)| {
            if !required_vars.contains_key(name) {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    if !unknown_vars.is_empty() {
        eprintln!(
            "Environment contains unspecified in 'VARS' array variables, therefore the following variables are not being checked:\n{:?}",
            unknown_vars
        );
    }
}

fn verify_types() {}
