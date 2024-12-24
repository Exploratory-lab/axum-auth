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
// todo:
pub fn validate_all(
    var_prefix: &str,
    vars: &[EnvVar],
) -> Result<HashMap<String, String>, AppError> {
    // Collect variables from the loaded environment that start
    // with the specified prefix
    let mut loaded_vars_with_prefix: HashMap<String, String> = std::env::vars()
        .filter(|(key, _)| key.starts_with(var_prefix))
        .collect();

    // Append required variable names with the specified prefix
    let required_vars_with_prefix: HashMap<String, EnvVar> = vars
        .iter()
        .map(|var| (format!("{}{}", var_prefix, var.name), *var))
        .collect();

    // Will remove unknown keys from the loaded variables, therefore
    // 'loaded_vars_with_prefix' will only have variables that are
    // specified in the 'required_vars_with_prefix'
    check_unknown(&mut loaded_vars_with_prefix, &required_vars_with_prefix);

    // Will throw an error and stop execution if any of the required
    // variables are missing from the loaded environment, therefore
    // both 'loaded_vars_with_prefix' and 'required_vars_with_prefix'
    // will have the same keys
    check_missing(&loaded_vars_with_prefix, &required_vars_with_prefix)?;

    verify_types(&loaded_vars_with_prefix, &required_vars_with_prefix)?;

    let constructed_vars: HashMap<String, String> =
        construct_variables(loaded_vars_with_prefix, required_vars_with_prefix);

    Ok(constructed_vars)
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

fn check_unknown(
    loaded_vars: &mut HashMap<String, String>,
    required_vars: &HashMap<String, EnvVar>,
) {
    let unknown_keys: HashSet<String> = loaded_vars
        .keys()
        .filter(|name| !required_vars.contains_key(*name))
        .cloned()
        .collect();

    if !unknown_keys.is_empty() {
        // Remove unknown keys from the loaded variables
        for key in &unknown_keys {
            loaded_vars.remove(key);
        }

        eprintln!(
            "Environment contains unspecified in 'VARS' array variables, therefore the following variables are not being checked:\n{:?}",
            unknown_keys
        );
    }
}

fn verify_types(
    loaded_vars: &HashMap<String, String>,
    required_vars: &HashMap<String, EnvVar>,
) -> Result<(), AppError> {
    if loaded_vars.len() != required_vars.len() {
        let kind: ErrorKind = ErrorKind::Env;
        let message: String = format!(
            "Loaded environment variables count ({}) does not match required variables count({})",
            loaded_vars.len(),
            required_vars.len()
        );

        return Err(AppError {
            kind,
            message,
            source: None,
        });
    }

    for (name, var_data) in required_vars {
        let value: &String = loaded_vars.get(name).unwrap();
        let var_type: &EnvVarType = &var_data.val_type;

        var_type.verify(value)?;
    }

    Ok(())
}

fn construct_variables(
    loaded_vars: HashMap<String, String>,
    required_vars: HashMap<String, EnvVar>,
) -> HashMap<String, String> {
    let mut constructed_vars: HashMap<String, String> = HashMap::new();

    for (name, var_data) in required_vars {
        let value: &String = loaded_vars.get(&name).unwrap();
        constructed_vars.insert(var_data.name.to_string(), value.to_string());
    }

    constructed_vars
}
