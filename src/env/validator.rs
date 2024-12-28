//! Module that contains functions for validating
//! loaded environment variables.

// Importing external crates
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Importing local modules
use super::constants::EnvVarType;
use crate::err::{AppError, ErrorKind};

pub trait EnvVar {
    type VarType; // Associated type for the type implementing the trait

    fn all() -> HashSet<Self::VarType>
    where
        Self: Sized;

    fn name(&self) -> String;

    fn value(&self) -> String;

    fn type_(&self) -> EnvVarType;

    fn verify(&self) -> Result<(), AppError>;

    fn verify_all() -> Result<(), AppError>;
}

/// ## Validates loaded environment variables.
///
/// Function validates loaded environment variables
/// against specified array of environment variables.
///
/// ## Examples
/// ```
/// // todo
/// ```
///
///
/// ## Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars_to_validate`: Variables to validate against.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If all required variables are present and
///       have correct types.
///     - `AppError`: Error type that contains error kind,
///       message and source.
pub fn validate<V>(var_prefix: &str, vars_to_validate: HashSet<V>) -> Result<(), AppError>
where
    V: EnvVar,             // HashSet of the type that implements the EnvVar trait
    V::VarType: Eq + Hash, // Ensure that the type can be used in a HashSet
{
    // Build a map of the variables to validate for
    // easier access and validation
    let vars_to_validate_map = vars_to_validate
        .iter()
        .map(|var| (var.name(), var))
        .collect();

    // Compare variables to validate with the loaded
    // environment variables, i.e. check if all required
    // variables are present and if there are any unknown
    compare_required_with_process_env(var_prefix, &vars_to_validate_map)?;

    // Verify the types of the loaded environment variables
    verify_types(&vars_to_validate_map)?;

    Ok(())
}

/// ## Checks for missing environment variables.
///
/// Function checks if any of the required environment
/// variables are missing from the loaded environment
/// variables.
///
/// ## Parameters
/// - `var_prefix`: Prefix for environment variables.
/// - `vars_to_validate`: Variables to validate against.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If no missing variables are found.
///     - `AppError`: If missing variables are found.
fn compare_required_with_process_env<V>(
    var_prefix: &str,
    vars_to_validate: &HashMap<String, &V>,
) -> Result<(), AppError>
where
    V: EnvVar,             // HashSet of the type that implements the EnvVar trait
    V::VarType: Eq + Hash, // Ensure that the type can be used in a HashSet
{
    // Collect variables from the loaded environment that start
    // with the specified prefix
    let loaded_vars_with_prefix: HashMap<String, String> = collect_app_vars(var_prefix);

    check_unknown(&loaded_vars_with_prefix, vars_to_validate)?;

    check_missing(&loaded_vars_with_prefix, vars_to_validate)?;

    Ok(())
}

/// ## Checks for unknown environment variables.
///
/// Function checks if there are any unknown environment
/// variables in the loaded environment variables.
///
/// ## Parameters
/// - `loaded_vars`: Loaded environment variables.
/// - `vars_to_validate`: Variables to validate against.
///
/// ## Returns
/// + `Result<(), AppError>`
///    - `()`: If no unknown variables are found.
///    - `AppError`: If unknown variables are found.
fn check_unknown<V>(
    loaded_vars: &HashMap<String, String>,
    vars_to_validate: &HashMap<String, &V>,
) -> Result<(), AppError>
where
    V: EnvVar,             // HashSet of the type that implements the EnvVar trait
    V::VarType: Eq + Hash, // Ensure that the type can be used in a HashSet
{
    // Collect the keys of the unknown loaded environment variables
    let unknown_vars: Vec<&str> = loaded_vars
        .keys()
        .filter_map(|key| {
            if !vars_to_validate.contains_key(key) {
                Some(key.as_str())
            } else {
                None
            }
        })
        .collect();

    // If there are unknown variables, return an error
    if !unknown_vars.is_empty() {
        let kind = ErrorKind::Env;
        let message = format!(
            "Unknown environment variables: '{}'",
            unknown_vars.join(", ")
        );
        let source = None;

        return Err(AppError::new(kind, message, source));
    }

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
/// - `vars_to_validate`: Variables to validate against.
///
/// ## Returns
/// + `Result<(), AppError>`
///    - `()`: If no missing variables are found.
///    - `AppError`: If missing variables are found.
fn check_missing<V>(
    loaded_vars: &HashMap<String, String>,
    vars_to_validate: &HashMap<String, &V>,
) -> Result<(), AppError>
where
    V: EnvVar,             // HashSet of the type that implements the EnvVar trait
    V::VarType: Eq + Hash, // Ensure that the type can be used in a HashSet
{
    let missing_vars: Vec<&str> = vars_to_validate
        .keys()
        .filter_map(|key| {
            if !loaded_vars.contains_key(key) {
                Some(key.as_str())
            } else {
                None
            }
        })
        .collect();

    if !missing_vars.is_empty() {
        let kind = ErrorKind::Env;
        let message = format!(
            "Missing environment variables: '{}'",
            missing_vars.join(", ")
        );
        let source = None;

        return Err(AppError::new(kind, message, source));
    }

    Ok(())
}

/// ## Verifies the types of the loaded environment variables.
///
/// Function verifies the types of the loaded environment
/// variables against the specified variables to validate.
///
/// ## Parameters
/// - `vars_to_validate`: Variables to validate against.
///
/// ## Returns
/// + `Result<(), AppError>`
///     - `()`: If types of all variables are correct.
///     - `AppError`: If any variable has an invalid type.
fn verify_types<V>(vars_to_validate: &HashMap<String, &V>) -> Result<(), AppError>
where
    V: EnvVar,             // HashSet of the type that implements the EnvVar trait
    V::VarType: Eq + Hash, // Ensure that the type can be used in a HashSet
{
    for (_, var_data) in vars_to_validate {
        var_data.verify()?;
    }
    Ok(())
}

/// ## Collects environment variables that start with a prefix.
///
/// Function collects environment variables that start with
/// a specified prefix.
///
/// ## Parameters
/// - `var_prefix`: Prefix for environment variables.
///
/// ## Returns
/// - `HashMap<String, String>`: Environment variables that start
fn collect_app_vars(var_prefix: &str) -> HashMap<String, String> {
    std::env::vars()
        .filter(|(key, _)| key.starts_with(var_prefix))
        .collect()
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
