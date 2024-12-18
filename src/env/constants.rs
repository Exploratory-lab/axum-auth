//! Constants for application environment setup.

// Local imports
use super::EnvVar;

// Environment file path to load
pub const FILE_PATH: &str = ".env";

// Prefix for environment variables
pub const PREFIX: &str = "AXA_";

// Environment variables to validate
pub const VARS: [EnvVar; 4] = [
    EnvVar {
        name: "DB_NAME",
        validator: None,
    },
    EnvVar {
        name: "DB_HOST",
        validator: None,
    },
    EnvVar {
        name: "DB_PORT",
        validator: None,
    },
    EnvVar {
        name: "DB_USER",
        validator: None,
    },
];
