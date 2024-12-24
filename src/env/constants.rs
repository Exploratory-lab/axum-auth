//! Constants for application environment setup.

// Local imports
use super::{EnvVar, EnvVarType};
use crate::strings::env::vars::{
    DB_HOST, DB_HOST_EXAMPLE, DB_NAME, DB_NAME_EXAMPLE, DB_PASS, DB_PASS_EXAMPLE, DB_PORT,
    DB_PORT_EXAMPLE, DB_SSL_MODE, DB_SSL_MODE_EXAMPLE, DB_USER, DB_USER_EXAMPLE,
    PATH_TO_DB_SSL_ROOT_CERT, PATH_TO_DB_SSL_ROOT_CERT_EXAMPLE,
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
        val_example: DB_NAME_EXAMPLE,
    },
    EnvVar {
        name: DB_HOST,
        val_type: EnvVarType::String,
        val_example: DB_HOST_EXAMPLE,
    },
    EnvVar {
        name: DB_PORT,
        val_type: EnvVarType::U16,
        val_example: DB_PORT_EXAMPLE,
    },
    EnvVar {
        name: DB_USER,
        val_type: EnvVarType::String,
        val_example: DB_USER_EXAMPLE,
    },
    EnvVar {
        name: DB_PASS,
        val_type: EnvVarType::String,
        val_example: DB_PASS_EXAMPLE,
    },
    EnvVar {
        name: DB_SSL_MODE,
        val_type: EnvVarType::Enum(&["disable", "require", "verify-ca", "verify-full"]),
        val_example: DB_SSL_MODE_EXAMPLE,
    },
    EnvVar {
        name: PATH_TO_DB_SSL_ROOT_CERT,
        val_type: EnvVarType::String,
        val_example: PATH_TO_DB_SSL_ROOT_CERT_EXAMPLE,
    },
];
