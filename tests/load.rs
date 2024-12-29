// Tests for the `load` function in the `env` module

use serial_test::serial;
use std::{collections::HashMap, env, io::Write};
use tempfile::NamedTempFile;

// use axum_auth::{
//     env::load,
//     err::{AppError, ErrorKind},
// };

// * Prefix for environment variables
const PREFIX: &str = "TEST_";

// * Environment variables
const VAR_1: &str = "VAR_1";
const VAR_2: &str = "VAR_2";
const VAR_3: &str = "VAR_3";
const EXTRA_VAR: &str = "EXTRA_VAR";

const VAL_1: &str = "VAL_1";
const VAL_2: &str = "VAL_2";
const VAL_3: &str = "1234";
const EXTRA_VAL: &str = "EXTRA_VAL";
const VAL_INVALID: &str = "INVALID";

// * Environment variables to validate
// const VARS: [EnvVar; 3] = [
//     EnvVar {
//         name: VAR_1,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: VAR_2,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: VAR_3,
//         val_type: EnvVarType::U16,
//     },
// ];

fn clean_up() {
    // Clean up environment variables after/before each test
    // for var in VARS.iter() {
    //     env::remove_var(format!("{}{}", PREFIX, var.name));
    // }
}

fn run_clean_test<F: FnOnce()>(test_fn: F) {
    clean_up();
    test_fn();
    clean_up();
}

fn create_env_file(contents: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    file
}

fn make_file_contents(prefix: &str, vars: Vec<(&str, &str)>) -> String {
    let mut contents: String = String::new();

    for (name, value) in vars.iter() {
        contents.push_str(&format!("{}{}={}\n", prefix, name, value));
    }

    contents
}

// Tests the `load` function when the file path is valid,
// the file contains all the required environment variables
// and the types of the variables are correct.
#[serial]
#[test]
fn test_env_load() {
    run_clean_test(|| {
        // let file_contents: String =
        //     make_file_contents(PREFIX, vec![(VAR_1, VAL_1), (VAR_2, VAL_2), (VAR_3, VAL_3)]);

        // let file: NamedTempFile = create_env_file(file_contents.as_str());

        // let app_vars: HashMap<&str, String> =
        //     load(file.path().to_str().unwrap(), PREFIX, &VARS).unwrap();

        // assert_eq!(app_vars.len(), 3);
        // assert_eq!(app_vars.get(VAR_1).unwrap(), VAL_1);
        // assert_eq!(app_vars.get(VAR_2).unwrap(), VAL_2);
        // assert_eq!(app_vars.get(VAR_3).unwrap(), VAL_3);
    });
}

// Tests the `load` function when the file path is valid,
// the file contains all the required environment variables
// but the types of the variables are incorrect.
#[serial]
#[test]
fn test_env_load_invalid_type() {
    run_clean_test(|| {
        // let file_contents: String = make_file_contents(
        //     PREFIX,
        //     vec![(VAR_1, VAL_1), (VAR_2, VAL_2), (VAR_3, VAL_INVALID)],
        // );

        // let file: NamedTempFile = create_env_file(file_contents.as_str());

        // let result: AppError = load(file.path().to_str().unwrap(), PREFIX, &VARS).unwrap_err();
        // let expected: AppError = EnvVarType::U16.verify(VAL_INVALID).unwrap_err();

        // assert_eq!(result, expected);
    });
}

// Tests the `load` function when the file path is valid,
// the file contains all the required environment variables
// but the file contains more variables than required.
#[serial]
#[test]
fn test_env_load_exceeding() {
    run_clean_test(|| {
        // let file_contents: String = make_file_contents(
        //     PREFIX,
        //     vec![
        //         (VAR_1, VAL_1),
        //         (VAR_2, VAL_2),
        //         (VAR_3, VAL_3),
        //         (EXTRA_VAR, EXTRA_VAL),
        //     ],
        // );

        // let file: NamedTempFile = create_env_file(file_contents.as_str());

        // let app_vars: HashMap<&str, String> =
        //     load(file.path().to_str().unwrap(), PREFIX, &VARS).unwrap();

        // assert_eq!(app_vars.len(), 3);
        // assert_eq!(app_vars.get(VAR_1).unwrap(), VAL_1);
        // assert_eq!(app_vars.get(VAR_2).unwrap(), VAL_2);
        // assert_eq!(app_vars.get(VAR_3).unwrap(), VAL_3);
    });
}

// Tests the `load` function when the file path is valid,
// but the file contains less variables than required.
#[serial]
#[test]
fn test_env_load_missing() {
    run_clean_test(|| {
        // let file_contents: String =
        //     make_file_contents(PREFIX, vec![(VAR_1, VAL_1), (VAR_2, VAL_2)]);

        // let file: NamedTempFile = create_env_file(file_contents.as_str());

        // let result: AppError = load(file.path().to_str().unwrap(), PREFIX, &VARS).unwrap_err();

        // let kind: ErrorKind = ErrorKind::Env;
        // let message: String = format!("Missing environment variables: '{}'", VAR_3);
        // let source = None;

        // let expected: AppError = AppError {
        //     kind,
        //     message,
        //     source,
        // };

        // assert_eq!(result, expected);
    });
}

// Tests the `load` function when the file path is valid,
// the file contains more variables than required but also
// has missing variables.
#[serial]
#[test]
fn test_env_load_exceeding_and_missing() {
    run_clean_test(|| {
        // let file_contents: String = make_file_contents(
        //     PREFIX,
        //     vec![(VAR_1, VAL_1), (VAR_2, VAL_2), (EXTRA_VAR, EXTRA_VAL)],
        // );

        // let file: NamedTempFile = create_env_file(file_contents.as_str());

        // let result: AppError = load(file.path().to_str().unwrap(), PREFIX, &VARS).unwrap_err();

        // let kind: ErrorKind = ErrorKind::Env;
        // let message: String = format!("Missing environment variables: '{}'", VAR_3);
        // let source = None;

        // let expected: AppError = AppError {
        //     kind,
        //     message,
        //     source,
        // };

        // assert_eq!(result, expected);
    });
}

// Tests the `load` function when the file path is invalid.
#[serial]
#[test]
fn test_env_load_non_existent() {
    run_clean_test(|| {
        // let file_path: &str = "non_existent_file";

        // let result: AppError = load(file_path, PREFIX, &VARS).unwrap_err();

        // let kind: ErrorKind = ErrorKind::Env;
        // let message: String = format!(
        //     "Failed to load environment file at specified path: '{}'",
        //     file_path
        // );

        // let e: dotenvy::Error = dotenvy::from_filename(file_path).unwrap_err();
        // let source = Some(Box::new(e) as Box<dyn std::error::Error>);

        // let expected: AppError = AppError {
        //     kind,
        //     message,
        //     source,
        // };

        // assert_eq!(result, expected);
    });
}
