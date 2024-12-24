//! Constants for application environment setup.

// Local imports
use super::{EnvVar, EnvVarType};

// Environment file path to load
pub const FILE_PATH: &str = ".env";

// Prefix for environment variables
pub const PREFIX: &str = "AXA_";

// Environment variables to validate
pub const VARS: [EnvVar; 4] = [
    EnvVar {
        name: DB_NAME,
        val_type: EnvVarType::String,
        val_example: "test_db",
    },
    EnvVar {
        name: DB_HOST,
        val_type: EnvVarType::String,
        val_example: "localhost",
    },
    EnvVar {
        name: DB_PORT,
        val_type: EnvVarType::U16,
        val_example: "5432",
    },
    EnvVar {
        name: DB_USER,
        val_type: EnvVarType::String,
        val_example: "test_user",
    },
    // EnvVar {
    //     name: "DB_HOLA",
    //     val_type: EnvVarType::String,
    //     val_example: "test_user",
    // },
];

pub const DB_NAME: &str = "DB_NAME";

pub const DB_HOST: &str = "DB_HOST";

pub const DB_PORT: &str = "DB_PORT";

pub const DB_USER: &str = "DB_USER";
