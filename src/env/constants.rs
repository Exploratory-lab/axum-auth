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

// * Environment file path to load
pub const FILE_PATH: &str = ".env";

// * Prefix for environment variables
pub const PREFIX: &str = "AXA_";

// * Environment variables to validate
// * keep it up to date with the .env.example,
// * .env files and
// pub const VARS: [EnvVar; 8] = [
//     EnvVar {
//         name: EnvVarName::DbName,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: EnvVarName::DbHost,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: EnvVarName::DbPort,
//         val_type: EnvVarType::U16,
//     },
//     EnvVar {
//         name: EnvVarName::DbUser,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: EnvVarName::DbPass,
//         val_type: EnvVarType::String,
//     },
//     EnvVar {
//         name: EnvVarName::DbSslMode,
//         val_type: EnvVarType::Enum(&[
//             DISABLE_SSL,
//             ALLOW_SSL,
//             PREFER_SSL,
//             REQUIRE_SSL,
//             VERIFY_CA_SSL,
//             VERIFY_FULL_SSL,
//         ]),
//     },
//     EnvVar {
//         name: EnvVarName::PathToDbSslRootCert,
//         val_type: EnvVarType::FilePath,
//     },
//     EnvVar {
//         name: EnvVarName::Test,
//         val_type: EnvVarType::String,
//     },
// ];

// todo: docs and tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum EnvVar {
    // Test, // !! delete
    DbName,
    DbHost,
    DbPort,
    DbUser,
    DbPass,
    DbSslMode,
    PathToDbSslRootCert,
}

impl EnvVar {
    const ERR_KIND: ErrorKind = ErrorKind::Env;

    pub fn all() -> HashSet<Self> {
        Self::iter().collect()
    }

    pub fn name(&self) -> String {
        match self {
            // Self::Test => Self::construct_name(PREFIX, "TEST"), // !! delete
            Self::DbName => Self::construct_name(PREFIX, DB_NAME),
            Self::DbHost => Self::construct_name(PREFIX, DB_HOST),
            Self::DbPort => Self::construct_name(PREFIX, DB_PORT),
            Self::DbUser => Self::construct_name(PREFIX, DB_USER),
            Self::DbPass => Self::construct_name(PREFIX, DB_PASS),
            Self::DbSslMode => Self::construct_name(PREFIX, DB_SSL_MODE),
            Self::PathToDbSslRootCert => Self::construct_name(PREFIX, PATH_TO_DB_SSL_ROOT_CERT),
        }
    }

    pub fn value(&self) -> String {
        std::env::var(self.name()).expect("Failed to get env var value")
    }

    pub fn type_(&self) -> EnvVarType {
        match self {
            // Self::Test => EnvVarType::String, // !! delete
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

    pub fn verify(&self) -> Result<(), AppError> {
        match self.type_() {
            EnvVarType::String => {
                if self.value().is_empty() {
                    let message = format!("Value cannot be empty for type: {:?}", self);
                    Err(AppError::new(Self::ERR_KIND, message, None))
                } else {
                    Ok(())
                }
            }
            EnvVarType::U16 => match is_u16(self.value().as_str(), Some(Self::ERR_KIND)) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            EnvVarType::Enum(allowed_values) => {
                if allowed_values.contains(&self.value().as_str()) {
                    Ok(())
                } else {
                    let message: String = format!(
                        "Value '{}' is not allowed for type {:?}",
                        self.value(),
                        self
                    );
                    let source: Option<Box<dyn error::Error>> = None;

                    Err(AppError::new(Self::ERR_KIND, message, source))
                }
            }
            EnvVarType::FilePath => {
                // todo: implement & test
                Ok(())
            }
        }
    }

    pub fn verify_all() -> Result<(), AppError> {
        let vars: HashSet<EnvVar> = Self::all();

        for var in vars {
            var.verify()?;
        }

        Ok(())
    }

    fn construct_name(prefix: &str, name: &str) -> String {
        format!("{}{}", prefix, name)
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
/// - `Enum`: Enum type environment variable with allowed values.
/// - `FilePath`: File path type environment variable.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvVarType {
    String,
    U16,
    Enum(&'static [&'static str]),
    FilePath,
}
