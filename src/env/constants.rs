//! Constants for application environment setup.

// External crate imports
use std::{collections::HashSet, error};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Local imports
use crate::{
    err::{AppError, ErrorKind},
    prelude::is_u16,
    strings::{
        env::vars::{
            DB_HOST, DB_NAME, DB_PASS, DB_PORT, DB_SSL_MODE, DB_USER, PATH_TO_DB_SSL_ROOT_CERT,
        },
        postgres::{
            ALLOW_SSL, DISABLE_SSL, PREFER_SSL, REQUIRE_SSL, VERIFY_CA_SSL, VERIFY_FULL_SSL,
        },
    },
};

use super::validator::EnvVar;

// * Environment file path to load
pub const FILE_PATH: &str = ".env";

// * Prefix for environment variables
pub const PREFIX: &str = "AXA_";

// * Environment variables to validate
// * keep it up to date with the .env.example,
// * .env files and
// todo: docs and tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum RequiredEnvVar {
    // Test, // !! delete
    AppMode,
    DbName,
    DbHost,
    DbPort,
    DbUser,
    DbPass,
    DbSslMode,
    PathToDbSslRootCert,
}

impl EnvVar for RequiredEnvVar {
    type VarType = Self;

    fn all() -> HashSet<Self> {
        Self::iter().collect()
    }

    fn name(&self) -> String {
        match self {
            // Self::Test => Self::construct_name(PREFIX, "TEST"), // !! delete
            Self::AppMode => construct_name(PREFIX, "APP_MODE"),
            Self::DbName => construct_name(PREFIX, DB_NAME),
            Self::DbHost => construct_name(PREFIX, DB_HOST),
            Self::DbPort => construct_name(PREFIX, DB_PORT),
            Self::DbUser => construct_name(PREFIX, DB_USER),
            Self::DbPass => construct_name(PREFIX, DB_PASS),
            Self::DbSslMode => construct_name(PREFIX, DB_SSL_MODE),
            Self::PathToDbSslRootCert => construct_name(PREFIX, PATH_TO_DB_SSL_ROOT_CERT),
        }
    }

    fn value(&self) -> String {
        std::env::var(self.name()).expect("Failed to get env var value")
    }

    fn type_(&self) -> EnvVarType {
        match self {
            // Self::Test => EnvVarType::String, // !! delete
            Self::AppMode => EnvVarType::Enum(&["dev", "prod"]),
            Self::DbName => EnvVarType::String,
            Self::DbHost => EnvVarType::String,
            Self::DbPort => EnvVarType::U16,
            Self::DbUser => EnvVarType::String,
            Self::DbPass => EnvVarType::String,
            Self::DbSslMode => EnvVarType::Enum(&[
                DISABLE_SSL,
                ALLOW_SSL,
                PREFER_SSL,
                REQUIRE_SSL,
                VERIFY_CA_SSL,
                VERIFY_FULL_SSL,
            ]),
            Self::PathToDbSslRootCert => EnvVarType::FilePath,
        }
    }

    fn verify(&self) -> Result<(), AppError> {
        const ERR_KIND: ErrorKind = ErrorKind::Env;

        match self.type_() {
            // todo: make helper functions for these types
            EnvVarType::String => {
                if self.value().is_empty() {
                    let message = format!("Value cannot be empty for type: {:?}", self.type_());

                    return Err(AppError::new(ERR_KIND, message, None));
                } else {
                    return Ok(());
                }
            }
            EnvVarType::U16 => match is_u16(self.value().as_str(), Some(ERR_KIND)) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            },
            EnvVarType::Enum(allowed_values) => {
                if allowed_values.contains(&self.value().as_str()) {
                    Ok(())
                } else {
                    let message: String = format!(
                        "Value '{}' is not allowed for type {:?}",
                        self.value(),
                        self.type_()
                    );
                    let source: Option<Box<dyn error::Error>> = None;

                    Err(AppError::new(ERR_KIND, message, source))
                }
            }
            EnvVarType::FilePath => {
                // todo: implement & test
                Ok(())
            }
        }
    }

    fn verify_all() -> Result<(), AppError> {
        let vars: HashSet<Self> = Self::all();

        for var in vars {
            var.verify()?;
        }

        Ok(())
    }

    // fn construct_name(prefix: &str, name: &str) -> String {
    //     format!("{}{}", prefix, name)
    // }
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
/// - `Enum`: Enum type environment variable with allowed values.
/// - `FilePath`: File path type environment variable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvVarType {
    String,
    U16,
    Enum(&'static [&'static str]),
    FilePath,
}

fn construct_name(prefix: &str, name: &str) -> String {
    format!("{}{}", prefix, name)
}
