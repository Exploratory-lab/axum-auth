//! Constants for application environment setup.

// Local imports
use super::{EnvVar, EnvVarType};
use crate::strings::{
    env::vars::{
        DB_HOST, DB_NAME, DB_PASS, DB_PORT, DB_SSL_MODE, DB_USER, PATH_TO_DB_SSL_ROOT_CERT,
    },
    postgres::{ALLOW_SSL, DISABLE_SSL, PREFER_SSL, REQUIRE_SSL, VERIFY_CA_SSL, VERIFY_FULL_SSL},
};

// * Environment file path to load
pub const FILE_PATH: &str = ".env";

// * Prefix for environment variables
pub const PREFIX: &str = "AXA_";

// * Environment variables to validate
// * keep it up to date with the .env.example
// * and .env files
pub const VARS: [EnvVar; 7] = [
    EnvVar {
        name: DB_NAME,
        val_type: EnvVarType::String,
    },
    EnvVar {
        name: DB_HOST,
        val_type: EnvVarType::String,
    },
    EnvVar {
        name: DB_PORT,
        val_type: EnvVarType::U16,
    },
    EnvVar {
        name: DB_USER,
        val_type: EnvVarType::String,
    },
    EnvVar {
        name: DB_PASS,
        val_type: EnvVarType::String,
    },
    EnvVar {
        name: DB_SSL_MODE,
        val_type: EnvVarType::Enum(&[
            DISABLE_SSL,
            ALLOW_SSL,
            PREFER_SSL,
            REQUIRE_SSL,
            VERIFY_CA_SSL,
            VERIFY_FULL_SSL,
        ]),
    },
    EnvVar {
        name: PATH_TO_DB_SSL_ROOT_CERT,
        val_type: EnvVarType::FilePath,
    },
];
